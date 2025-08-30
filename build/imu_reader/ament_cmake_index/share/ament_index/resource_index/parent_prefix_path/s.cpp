#include <rclcpp/rclcpp.hpp>
#include <sensor_msgs/msg/imu.hpp>
#include <sensor_msgs/msg/temperature.hpp>

#include <linux/i2c-dev.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <cerrno>
#include <cstring>

#define MPU_ADDR        0x68
#define REG_PWR_MGMT_1  0x6B
#define REG_CONFIG      0x1A
#define REG_GYRO_CFG    0x1B
#define REG_ACC_CFG     0x1C
#define REG_DATA        0x3B

// 与寄存器设置匹配：ACC=0x10(±8g) / GYRO=0x18(±2000 dps)
static constexpr double ACC_LSB_PER_G   = 4096.0; // ±8g
static constexpr double GYR_LSB_PER_DPS = 16.4;   // ±2000 dps
static constexpr double TEMP_SCALE      = 340.0;
static constexpr double TEMP_OFFSET     = 36.53;

// 常量
static constexpr double G0 = 9.80665;
static constexpr double DEG2RAD = M_PI / 180.0;

// ====== 固定补偿参数（写死） ======
// 你观测到的零偏：Y 轴 −0.75 deg/s，Z 轴 −1.25 deg/s（X 按 0）
// 这里以“rad/s”存储，并在发布前做：omega_corr = omega_raw_rad - GYR_BIAS_*
static constexpr double GYR_BIAS_X = 0.0;                        // 0.00 deg/s
static constexpr double GYR_BIAS_Y = (-0.75) * DEG2RAD;          // ≈ -0.013089969 rad/s
static constexpr double GYR_BIAS_Z = (-1.25) * DEG2RAD;          // ≈ -0.021816616 rad/s

// 陀螺温漂一次项（rad/s/°C），暂不使用写 0
static constexpr double GYR_K_X = 0.0;
static constexpr double GYR_K_Y = 0.0;
static constexpr double GYR_K_Z = 0.0;

// 加速度计零偏（m/s^2）与温漂（m/s^2/°C），不写死则留 0
static constexpr double ACC_BIAS_X = 0.0;
static constexpr double ACC_BIAS_Y = 0.0;
static constexpr double ACC_BIAS_Z = 0.0;
static constexpr double ACC_K_X = 0.0;
static constexpr double ACC_K_Y = 0.0;
static constexpr double ACC_K_Z = 0.0;

static constexpr double TEMP_REF_C = 25.0;

class ImuReader : public rclcpp::Node {
public:
  ImuReader()
  : Node("imu_reader_fixed"),
    period_(1.0/400.0),
    i2c_bus_(-1)
  {
    imu_pub_  = this->create_publisher<sensor_msgs::msg::Imu>("imu/data_raw", 50);
    temp_pub_ = this->create_publisher<sensor_msgs::msg::Temperature>("imu/temperature", 10);

    const char* dev = "/dev/i2c-5";  // 按需改你的 I2C 口
    i2c_bus_ = ::open(dev, O_RDWR);
    if (i2c_bus_ < 0) {
      RCLCPP_ERROR(get_logger(), "open(%s) failed: %s", dev, std::strerror(errno));
      throw std::runtime_error("I2C open failed");
    }
    if (::ioctl(i2c_bus_, I2C_SLAVE, MPU_ADDR) < 0) {
      RCLCPP_ERROR(get_logger(), "ioctl(I2C_SLAVE) failed: %s", std::strerror(errno));
      throw std::runtime_error("I2C set slave failed");
    }

    if (!init_device()) {
      throw std::runtime_error("IMU init failed");
    }

    timer_ = this->create_wall_timer(
      5ms,
      std::bind(&ImuReader::read_data, this));
  }

  ~ImuReader() override {
    if (i2c_bus_ >= 0) ::close(i2c_bus_);
  }

private:
  bool write_byte(uint8_t reg, uint8_t value) {
    uint8_t buf[2] = {reg, value};
    if (::write(i2c_bus_, buf, 2) != 2) {
      RCLCPP_ERROR(get_logger(), "I2C write 0x%02X=0x%02X failed: %s",
                   reg, value, std::strerror(errno));
      return false;
    }
    return true;
  }

  bool init_device() {
    bool ok = true;
    ok &= write_byte(REG_PWR_MGMT_1, 0x01); // 唤醒，时钟源 XGYRO
    printf("ok1 %d\r\n",ok);
    ok &= write_byte(REG_CONFIG,     0x03); // DLPF=3 (~44Hz)
    printf("ok2 %d\r\n",ok);
    ok &= write_byte(REG_GYRO_CFG,   0x18); // ±2000 dps
    printf("ok3 %d\r\n",ok);
    ok &= write_byte(REG_ACC_CFG,    0x10); // ±8 g
    printf("ok4 %d\r\n",ok);
    return ok;
  }

  void read_data() {
    uint8_t reg = REG_DATA;
    if (::write(i2c_bus_, &reg, 1) != 1) {
      RCLCPP_ERROR(get_logger(), "select REG_DATA failed: %s", std::strerror(errno));
      return;
    }

    uint8_t raw[14];
    if (::read(i2c_bus_, raw, 14) != 14) {
      RCLCPP_ERROR(get_logger(), "I2C read failed: %s", std::strerror(errno));
      return;
    }

    // 原始寄存器值
    int16_t ax = (int16_t)((raw[0] << 8) | raw[1]);
    int16_t ay = (int16_t)((raw[2] << 8) | raw[3]);
    int16_t az = (int16_t)((raw[4] << 8) | raw[5]);
    int16_t tp = (int16_t)((raw[6] << 8) | raw[7]);
    int16_t gx = (int16_t)((raw[8] << 8) | raw[9]);
    int16_t gy = (int16_t)((raw[10] << 8) | raw[11]);
    int16_t gz = (int16_t)((raw[12] << 8) | raw[13]);

    // 工程单位（未补偿）
    double ax_g = ax / ACC_LSB_PER_G;
    double ay_g = ay / ACC_LSB_PER_G;
    double az_g = az / ACC_LSB_PER_G;

    double gx_dps = gx / GYR_LSB_PER_DPS;
    double gy_dps = gy / GYR_LSB_PER_DPS;
    double gz_dps = gz / GYR_LSB_PER_DPS;

    double T = tp / TEMP_SCALE + TEMP_OFFSET;

    // ===== 应用固定补偿 =====
    // 加速度（m/s^2）
    double ax_ms2 = ax_g * G0 - ACC_BIAS_X + ACC_K_X * (T - TEMP_REF_C);
    double ay_ms2 = ay_g * G0 - ACC_BIAS_Y + ACC_K_Y * (T - TEMP_REF_C);
    double az_ms2 = az_g * G0 - ACC_BIAS_Z + ACC_K_Z * (T - TEMP_REF_C);

    // 陀螺（rad/s）：减去你写死的每轴零偏，再加温漂一次项（此处为 0）
    double wx_rads = gx_dps * DEG2RAD - GYR_BIAS_X + GYR_K_X * (T - TEMP_REF_C);
    double wy_rads = gy_dps * DEG2RAD - GYR_BIAS_Y + GYR_K_Y * (T - TEMP_REF_C);
    double wz_rads = gz_dps * DEG2RAD - GYR_BIAS_Z + GYR_K_Z * (T - TEMP_REF_C);

    // ===== 发布 Imu =====
    auto now = this->get_clock()->now();

    sensor_msgs::msg::Imu imu;
    imu.header.stamp = now;
    imu.header.frame_id = "imu_link";

    imu.angular_velocity.x = wx_rads;
    imu.angular_velocity.y = wy_rads;
    imu.angular_velocity.z = wz_rads;
    imu.angular_velocity_covariance[0] = -1.0;  // 未提供噪声

    imu.linear_acceleration.x = ax_ms2;
    imu.linear_acceleration.y = ay_ms2;
    imu.linear_acceleration.z = az_ms2;
    imu.linear_acceleration_covariance[0] = -1.0;

    imu.orientation_covariance[0] = -1.0;       // 未提供姿态

    imu_pub_->publish(imu);

    // 可选：温度
    sensor_msgs::msg::Temperature tp_msg;
    tp_msg.header = imu.header;
    tp_msg.temperature = T;
    tp_msg.variance = 0.0;
    temp_pub_->publish(tp_msg);
  }

private:
  double period_;
  int i2c_bus_;
  rclcpp::TimerBase::SharedPtr timer_;
  rclcpp::Publisher<sensor_msgs::msg::Imu>::SharedPtr imu_pub_;
  rclcpp::Publisher<sensor_msgs::msg::Temperature>::SharedPtr temp_pub_;
};

int main(int argc, char **argv) {
  try {
    rclcpp::init(argc, argv);
    auto node = std::make_shared<ImuReader>();
    rclcpp::spin(node);
    rclcpp::shutdown();
  } catch (const std::exception& e) {
    fprintf(stderr, "Fatal: %s\n", e.what());
    return 1;
  }
  return 0;
}
