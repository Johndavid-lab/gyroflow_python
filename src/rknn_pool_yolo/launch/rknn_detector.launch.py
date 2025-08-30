from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    return LaunchDescription([
        Node(
            package='rknn_pool_yolo',
            executable='rknn_detector',
            name='rknn_detector',
            output='screen',
            parameters=[
                {'model_path': '/home/cat/yolov8s_i8.rknn'},
                {'num_workers': 12},
                {'score_thresh': 0.10},
                {'min_box': 16},
                {'subscribe_topic': '/image_raw'},
                {'publish_annotated': True},
                # {'class_names': ['person','bicycle', ...]},  # 可选
            ],
        )
    ])
