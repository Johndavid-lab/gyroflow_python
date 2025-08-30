pub mod msg {
    use super::super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct BoundingBox2D {
        pub center: vision_msgs::msg::Pose2D,
        pub size_x: f64,
        pub size_y: f64,
    }
    impl WrappedTypesupport for BoundingBox2D {
        type CStruct = vision_msgs__msg__BoundingBox2D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__BoundingBox2D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__BoundingBox2D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox2D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox2D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__BoundingBox2D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox2D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox2D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> BoundingBox2D {
            BoundingBox2D {
                center: vision_msgs::msg::Pose2D::from_native(&msg.center),
                size_x: msg.size_x,
                size_y: msg.size_y,
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.center.copy_to_native(&mut msg.center);
            msg.size_x = self.size_x;
            msg.size_y = self.size_y;
        }
    }
    impl Default for BoundingBox2D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<BoundingBox2D>::new();
            BoundingBox2D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct BoundingBox2DArray {
        pub header: std_msgs::msg::Header,
        pub boxes: Vec<vision_msgs::msg::BoundingBox2D>,
    }
    impl WrappedTypesupport for BoundingBox2DArray {
        type CStruct = vision_msgs__msg__BoundingBox2DArray;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__BoundingBox2DArray()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__BoundingBox2DArray {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox2DArray__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox2DArray__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__BoundingBox2DArray) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox2DArray__destroy(msg) };
            #[cfg(feature = "doc-only")]
            vision_msgs__msg__BoundingBox2DArray__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> BoundingBox2DArray {
            BoundingBox2DArray {
                header: std_msgs::msg::Header::from_native(&msg.header),
                boxes: {
                    let mut temp = Vec::with_capacity(msg.boxes.size);
                    if msg.boxes.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(msg.boxes.data, msg.boxes.size)
                        };
                        for s in slice {
                            temp.push(vision_msgs::msg::BoundingBox2D::from_native(s));
                        }
                    }
                    temp
                },
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__BoundingBox2D__Sequence__fini(&mut msg.boxes);
                vision_msgs__msg__BoundingBox2D__Sequence__init(
                    &mut msg.boxes,
                    self.boxes.len(),
                );
                if msg.boxes.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.boxes.data,
                        msg.boxes.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.boxes) {
                        s.copy_to_native(t);
                    }
                }
            }
        }
    }
    impl Default for BoundingBox2DArray {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<BoundingBox2DArray>::new();
            BoundingBox2DArray::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct BoundingBox3D {
        pub center: geometry_msgs::msg::Pose,
        pub size: geometry_msgs::msg::Vector3,
    }
    impl WrappedTypesupport for BoundingBox3D {
        type CStruct = vision_msgs__msg__BoundingBox3D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__BoundingBox3D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__BoundingBox3D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox3D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox3D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__BoundingBox3D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox3D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox3D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> BoundingBox3D {
            BoundingBox3D {
                center: geometry_msgs::msg::Pose::from_native(&msg.center),
                size: geometry_msgs::msg::Vector3::from_native(&msg.size),
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.center.copy_to_native(&mut msg.center);
            self.size.copy_to_native(&mut msg.size);
        }
    }
    impl Default for BoundingBox3D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<BoundingBox3D>::new();
            BoundingBox3D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct BoundingBox3DArray {
        pub header: std_msgs::msg::Header,
        pub boxes: Vec<vision_msgs::msg::BoundingBox3D>,
    }
    impl WrappedTypesupport for BoundingBox3DArray {
        type CStruct = vision_msgs__msg__BoundingBox3DArray;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__BoundingBox3DArray()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__BoundingBox3DArray {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox3DArray__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__BoundingBox3DArray__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__BoundingBox3DArray) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__BoundingBox3DArray__destroy(msg) };
            #[cfg(feature = "doc-only")]
            vision_msgs__msg__BoundingBox3DArray__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> BoundingBox3DArray {
            BoundingBox3DArray {
                header: std_msgs::msg::Header::from_native(&msg.header),
                boxes: {
                    let mut temp = Vec::with_capacity(msg.boxes.size);
                    if msg.boxes.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(msg.boxes.data, msg.boxes.size)
                        };
                        for s in slice {
                            temp.push(vision_msgs::msg::BoundingBox3D::from_native(s));
                        }
                    }
                    temp
                },
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__BoundingBox3D__Sequence__fini(&mut msg.boxes);
                vision_msgs__msg__BoundingBox3D__Sequence__init(
                    &mut msg.boxes,
                    self.boxes.len(),
                );
                if msg.boxes.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.boxes.data,
                        msg.boxes.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.boxes) {
                        s.copy_to_native(t);
                    }
                }
            }
        }
    }
    impl Default for BoundingBox3DArray {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<BoundingBox3DArray>::new();
            BoundingBox3DArray::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Classification {
        pub header: std_msgs::msg::Header,
        pub results: Vec<vision_msgs::msg::ObjectHypothesis>,
    }
    impl WrappedTypesupport for Classification {
        type CStruct = vision_msgs__msg__Classification;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Classification()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Classification {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Classification__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Classification__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Classification) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Classification__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Classification__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Classification {
            Classification {
                header: std_msgs::msg::Header::from_native(&msg.header),
                results: {
                    let mut temp = Vec::with_capacity(msg.results.size);
                    if msg.results.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.results.data,
                                msg.results.size,
                            )
                        };
                        for s in slice {
                            temp.push(
                                vision_msgs::msg::ObjectHypothesis::from_native(s),
                            );
                        }
                    }
                    temp
                },
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__ObjectHypothesis__Sequence__fini(&mut msg.results);
                vision_msgs__msg__ObjectHypothesis__Sequence__init(
                    &mut msg.results,
                    self.results.len(),
                );
                if msg.results.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.results.data,
                        msg.results.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.results) {
                        s.copy_to_native(t);
                    }
                }
            }
        }
    }
    impl Default for Classification {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Classification>::new();
            Classification::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Detection2D {
        pub header: std_msgs::msg::Header,
        pub results: Vec<vision_msgs::msg::ObjectHypothesisWithPose>,
        pub bbox: vision_msgs::msg::BoundingBox2D,
        pub id: std::string::String,
    }
    impl WrappedTypesupport for Detection2D {
        type CStruct = vision_msgs__msg__Detection2D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Detection2D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Detection2D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection2D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection2D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Detection2D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection2D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection2D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Detection2D {
            Detection2D {
                header: std_msgs::msg::Header::from_native(&msg.header),
                results: {
                    let mut temp = Vec::with_capacity(msg.results.size);
                    if msg.results.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.results.data,
                                msg.results.size,
                            )
                        };
                        for s in slice {
                            temp.push(
                                vision_msgs::msg::ObjectHypothesisWithPose::from_native(s),
                            );
                        }
                    }
                    temp
                },
                bbox: vision_msgs::msg::BoundingBox2D::from_native(&msg.bbox),
                id: msg.id.to_str().to_owned(),
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__ObjectHypothesisWithPose__Sequence__fini(
                    &mut msg.results,
                );
                vision_msgs__msg__ObjectHypothesisWithPose__Sequence__init(
                    &mut msg.results,
                    self.results.len(),
                );
                if msg.results.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.results.data,
                        msg.results.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.results) {
                        s.copy_to_native(t);
                    }
                }
            }
            self.bbox.copy_to_native(&mut msg.bbox);
            msg.id.assign(&self.id);
        }
    }
    impl Default for Detection2D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Detection2D>::new();
            Detection2D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Detection2DArray {
        pub header: std_msgs::msg::Header,
        pub detections: Vec<vision_msgs::msg::Detection2D>,
    }
    impl WrappedTypesupport for Detection2DArray {
        type CStruct = vision_msgs__msg__Detection2DArray;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Detection2DArray()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Detection2DArray {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection2DArray__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection2DArray__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Detection2DArray) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection2DArray__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection2DArray__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Detection2DArray {
            Detection2DArray {
                header: std_msgs::msg::Header::from_native(&msg.header),
                detections: {
                    let mut temp = Vec::with_capacity(msg.detections.size);
                    if msg.detections.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.detections.data,
                                msg.detections.size,
                            )
                        };
                        for s in slice {
                            temp.push(vision_msgs::msg::Detection2D::from_native(s));
                        }
                    }
                    temp
                },
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__Detection2D__Sequence__fini(&mut msg.detections);
                vision_msgs__msg__Detection2D__Sequence__init(
                    &mut msg.detections,
                    self.detections.len(),
                );
                if msg.detections.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.detections.data,
                        msg.detections.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.detections) {
                        s.copy_to_native(t);
                    }
                }
            }
        }
    }
    impl Default for Detection2DArray {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Detection2DArray>::new();
            Detection2DArray::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Detection3D {
        pub header: std_msgs::msg::Header,
        pub results: Vec<vision_msgs::msg::ObjectHypothesisWithPose>,
        pub bbox: vision_msgs::msg::BoundingBox3D,
        pub id: std::string::String,
    }
    impl WrappedTypesupport for Detection3D {
        type CStruct = vision_msgs__msg__Detection3D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Detection3D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Detection3D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection3D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection3D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Detection3D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection3D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection3D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Detection3D {
            Detection3D {
                header: std_msgs::msg::Header::from_native(&msg.header),
                results: {
                    let mut temp = Vec::with_capacity(msg.results.size);
                    if msg.results.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.results.data,
                                msg.results.size,
                            )
                        };
                        for s in slice {
                            temp.push(
                                vision_msgs::msg::ObjectHypothesisWithPose::from_native(s),
                            );
                        }
                    }
                    temp
                },
                bbox: vision_msgs::msg::BoundingBox3D::from_native(&msg.bbox),
                id: msg.id.to_str().to_owned(),
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__ObjectHypothesisWithPose__Sequence__fini(
                    &mut msg.results,
                );
                vision_msgs__msg__ObjectHypothesisWithPose__Sequence__init(
                    &mut msg.results,
                    self.results.len(),
                );
                if msg.results.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.results.data,
                        msg.results.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.results) {
                        s.copy_to_native(t);
                    }
                }
            }
            self.bbox.copy_to_native(&mut msg.bbox);
            msg.id.assign(&self.id);
        }
    }
    impl Default for Detection3D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Detection3D>::new();
            Detection3D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Detection3DArray {
        pub header: std_msgs::msg::Header,
        pub detections: Vec<vision_msgs::msg::Detection3D>,
    }
    impl WrappedTypesupport for Detection3DArray {
        type CStruct = vision_msgs__msg__Detection3DArray;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Detection3DArray()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Detection3DArray {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection3DArray__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection3DArray__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Detection3DArray) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Detection3DArray__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Detection3DArray__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Detection3DArray {
            Detection3DArray {
                header: std_msgs::msg::Header::from_native(&msg.header),
                detections: {
                    let mut temp = Vec::with_capacity(msg.detections.size);
                    if msg.detections.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.detections.data,
                                msg.detections.size,
                            )
                        };
                        for s in slice {
                            temp.push(vision_msgs::msg::Detection3D::from_native(s));
                        }
                    }
                    temp
                },
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__Detection3D__Sequence__fini(&mut msg.detections);
                vision_msgs__msg__Detection3D__Sequence__init(
                    &mut msg.detections,
                    self.detections.len(),
                );
                if msg.detections.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.detections.data,
                        msg.detections.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.detections) {
                        s.copy_to_native(t);
                    }
                }
            }
        }
    }
    impl Default for Detection3DArray {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Detection3DArray>::new();
            Detection3DArray::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct LabelInfo {
        pub header: std_msgs::msg::Header,
        pub class_map: Vec<vision_msgs::msg::VisionClass>,
        pub threshold: f32,
    }
    impl WrappedTypesupport for LabelInfo {
        type CStruct = vision_msgs__msg__LabelInfo;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__LabelInfo()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__LabelInfo {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__LabelInfo__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__LabelInfo__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__LabelInfo) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__LabelInfo__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__LabelInfo__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> LabelInfo {
            LabelInfo {
                header: std_msgs::msg::Header::from_native(&msg.header),
                class_map: {
                    let mut temp = Vec::with_capacity(msg.class_map.size);
                    if msg.class_map.data != std::ptr::null_mut() {
                        let slice = unsafe {
                            std::slice::from_raw_parts(
                                msg.class_map.data,
                                msg.class_map.size,
                            )
                        };
                        for s in slice {
                            temp.push(vision_msgs::msg::VisionClass::from_native(s));
                        }
                    }
                    temp
                },
                threshold: msg.threshold,
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            unsafe {
                vision_msgs__msg__VisionClass__Sequence__fini(&mut msg.class_map);
                vision_msgs__msg__VisionClass__Sequence__init(
                    &mut msg.class_map,
                    self.class_map.len(),
                );
                if msg.class_map.data != std::ptr::null_mut() {
                    let slice = std::slice::from_raw_parts_mut(
                        msg.class_map.data,
                        msg.class_map.size,
                    );
                    for (t, s) in slice.iter_mut().zip(&self.class_map) {
                        s.copy_to_native(t);
                    }
                }
            }
            msg.threshold = self.threshold;
        }
    }
    impl Default for LabelInfo {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<LabelInfo>::new();
            LabelInfo::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct ObjectHypothesis {
        pub class_id: std::string::String,
        pub score: f64,
    }
    impl WrappedTypesupport for ObjectHypothesis {
        type CStruct = vision_msgs__msg__ObjectHypothesis;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__ObjectHypothesis()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__ObjectHypothesis {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__ObjectHypothesis__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__ObjectHypothesis__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__ObjectHypothesis) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__ObjectHypothesis__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__ObjectHypothesis__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> ObjectHypothesis {
            ObjectHypothesis {
                class_id: msg.class_id.to_str().to_owned(),
                score: msg.score,
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            msg.class_id.assign(&self.class_id);
            msg.score = self.score;
        }
    }
    impl Default for ObjectHypothesis {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<ObjectHypothesis>::new();
            ObjectHypothesis::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct ObjectHypothesisWithPose {
        pub hypothesis: vision_msgs::msg::ObjectHypothesis,
        pub pose: geometry_msgs::msg::PoseWithCovariance,
    }
    impl WrappedTypesupport for ObjectHypothesisWithPose {
        type CStruct = vision_msgs__msg__ObjectHypothesisWithPose;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__ObjectHypothesisWithPose()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__ObjectHypothesisWithPose {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__ObjectHypothesisWithPose__create() }
            #[cfg(feature = "doc-only")]
            vision_msgs__msg__ObjectHypothesisWithPose__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__ObjectHypothesisWithPose) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__ObjectHypothesisWithPose__destroy(msg) };
            #[cfg(feature = "doc-only")]
            vision_msgs__msg__ObjectHypothesisWithPose__destroy(msg)
        }
        fn from_native(
            #[allow(unused)]
            msg: &Self::CStruct,
        ) -> ObjectHypothesisWithPose {
            ObjectHypothesisWithPose {
                hypothesis: vision_msgs::msg::ObjectHypothesis::from_native(
                    &msg.hypothesis,
                ),
                pose: geometry_msgs::msg::PoseWithCovariance::from_native(&msg.pose),
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.hypothesis.copy_to_native(&mut msg.hypothesis);
            self.pose.copy_to_native(&mut msg.pose);
        }
    }
    impl Default for ObjectHypothesisWithPose {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<ObjectHypothesisWithPose>::new();
            ObjectHypothesisWithPose::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Point2D {
        pub x: f64,
        pub y: f64,
    }
    impl WrappedTypesupport for Point2D {
        type CStruct = vision_msgs__msg__Point2D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Point2D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Point2D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Point2D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Point2D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Point2D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Point2D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Point2D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Point2D {
            Point2D { x: msg.x, y: msg.y }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            msg.x = self.x;
            msg.y = self.y;
        }
    }
    impl Default for Point2D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Point2D>::new();
            Point2D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Pose2D {
        pub position: vision_msgs::msg::Point2D,
        pub theta: f64,
    }
    impl WrappedTypesupport for Pose2D {
        type CStruct = vision_msgs__msg__Pose2D;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__Pose2D()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__Pose2D {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Pose2D__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__Pose2D__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__Pose2D) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__Pose2D__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__Pose2D__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> Pose2D {
            Pose2D {
                position: vision_msgs::msg::Point2D::from_native(&msg.position),
                theta: msg.theta,
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.position.copy_to_native(&mut msg.position);
            msg.theta = self.theta;
        }
    }
    impl Default for Pose2D {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<Pose2D>::new();
            Pose2D::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct VisionClass {
        pub class_id: u16,
        pub class_name: std::string::String,
    }
    impl WrappedTypesupport for VisionClass {
        type CStruct = vision_msgs__msg__VisionClass;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__VisionClass()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__VisionClass {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__VisionClass__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__VisionClass__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__VisionClass) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__VisionClass__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__VisionClass__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> VisionClass {
            VisionClass {
                class_id: msg.class_id,
                class_name: msg.class_name.to_str().to_owned(),
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            msg.class_id = self.class_id;
            msg.class_name.assign(&self.class_name);
        }
    }
    impl Default for VisionClass {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<VisionClass>::new();
            VisionClass::from_native(&msg_native)
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    pub struct VisionInfo {
        pub header: std_msgs::msg::Header,
        pub method: std::string::String,
        pub database_location: std::string::String,
        pub database_version: i32,
    }
    impl WrappedTypesupport for VisionInfo {
        type CStruct = vision_msgs__msg__VisionInfo;
        fn get_ts() -> &'static rosidl_message_type_support_t {
            unsafe {
                &*rosidl_typesupport_c__get_message_type_support_handle__vision_msgs__msg__VisionInfo()
            }
        }
        fn create_msg() -> *mut vision_msgs__msg__VisionInfo {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__VisionInfo__create() }
            #[cfg(feature = "doc-only")] vision_msgs__msg__VisionInfo__create()
        }
        fn destroy_msg(msg: *mut vision_msgs__msg__VisionInfo) -> () {
            #[cfg(not(feature = "doc-only"))]
            unsafe { vision_msgs__msg__VisionInfo__destroy(msg) };
            #[cfg(feature = "doc-only")] vision_msgs__msg__VisionInfo__destroy(msg)
        }
        fn from_native(#[allow(unused)] msg: &Self::CStruct) -> VisionInfo {
            VisionInfo {
                header: std_msgs::msg::Header::from_native(&msg.header),
                method: msg.method.to_str().to_owned(),
                database_location: msg.database_location.to_str().to_owned(),
                database_version: msg.database_version,
            }
        }
        fn copy_to_native(&self, #[allow(unused)] msg: &mut Self::CStruct) {
            self.header.copy_to_native(&mut msg.header);
            msg.method.assign(&self.method);
            msg.database_location.assign(&self.database_location);
            msg.database_version = self.database_version;
        }
    }
    impl Default for VisionInfo {
        fn default() -> Self {
            let msg_native = WrappedNativeMsg::<VisionInfo>::new();
            VisionInfo::from_native(&msg_native)
        }
    }
}
