impl UntypedServiceSupport {
    pub fn new_from(typename: &str) -> Result<Self> {
        #[allow(non_snake_case)]
        fn new_untyped_service_support_action_msgs_srv_CancelGoal() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<action_msgs::srv::CancelGoal::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_composition_interfaces_srv_ListNodes() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                composition_interfaces::srv::ListNodes::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_composition_interfaces_srv_LoadNode() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                composition_interfaces::srv::LoadNode::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_composition_interfaces_srv_UnloadNode() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                composition_interfaces::srv::UnloadNode::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_diagnostic_msgs_srv_AddDiagnostics() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<diagnostic_msgs::srv::AddDiagnostics::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_diagnostic_msgs_srv_SelfTest() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<diagnostic_msgs::srv::SelfTest::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_lifecycle_msgs_srv_ChangeState() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<lifecycle_msgs::srv::ChangeState::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_lifecycle_msgs_srv_GetAvailableStates() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                lifecycle_msgs::srv::GetAvailableStates::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_lifecycle_msgs_srv_GetAvailableTransitions() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                lifecycle_msgs::srv::GetAvailableTransitions::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_lifecycle_msgs_srv_GetState() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<lifecycle_msgs::srv::GetState::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_nav_msgs_srv_GetMap() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<nav_msgs::srv::GetMap::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_nav_msgs_srv_GetPlan() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<nav_msgs::srv::GetPlan::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_nav_msgs_srv_LoadMap() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<nav_msgs::srv::LoadMap::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_nav_msgs_srv_SetMap() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<nav_msgs::srv::SetMap::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_DescribeParameters() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                rcl_interfaces::srv::DescribeParameters::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_GetParameterTypes() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                rcl_interfaces::srv::GetParameterTypes::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_GetParameters() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rcl_interfaces::srv::GetParameters::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_ListParameters() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rcl_interfaces::srv::ListParameters::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_SetParameters() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rcl_interfaces::srv::SetParameters::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rcl_interfaces_srv_SetParametersAtomically() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                rcl_interfaces::srv::SetParametersAtomically::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_Burst() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::Burst::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_GetRate() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::GetRate::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_IsPaused() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::IsPaused::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_Pause() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::Pause::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_PlayNext() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::PlayNext::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_Resume() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::Resume::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_Seek() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::Seek::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_SetRate() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::SetRate::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_Snapshot() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<rosbag2_interfaces::srv::Snapshot::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_rosbag2_interfaces_srv_TogglePaused() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                rosbag2_interfaces::srv::TogglePaused::Service,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_sensor_msgs_srv_SetCameraInfo() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<sensor_msgs::srv::SetCameraInfo::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_std_srvs_srv_Empty() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<std_srvs::srv::Empty::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_std_srvs_srv_SetBool() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<std_srvs::srv::SetBool::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_std_srvs_srv_Trigger() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<std_srvs::srv::Trigger::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_tf2_msgs_srv_FrameGraph() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<tf2_msgs::srv::FrameGraph::Service>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_visualization_msgs_srv_GetInteractiveMarkers() -> UntypedServiceSupport {
            UntypedServiceSupport::new::<
                visualization_msgs::srv::GetInteractiveMarkers::Service,
            >()
        }
        static MAP: phf::Map<&'static str, fn() -> UntypedServiceSupport> = phf::phf_map! {
            "action_msgs/srv/CancelGoal" =>
            new_untyped_service_support_action_msgs_srv_CancelGoal,
            "composition_interfaces/srv/ListNodes" =>
            new_untyped_service_support_composition_interfaces_srv_ListNodes,
            "composition_interfaces/srv/LoadNode" =>
            new_untyped_service_support_composition_interfaces_srv_LoadNode,
            "composition_interfaces/srv/UnloadNode" =>
            new_untyped_service_support_composition_interfaces_srv_UnloadNode,
            "diagnostic_msgs/srv/AddDiagnostics" =>
            new_untyped_service_support_diagnostic_msgs_srv_AddDiagnostics,
            "diagnostic_msgs/srv/SelfTest" =>
            new_untyped_service_support_diagnostic_msgs_srv_SelfTest,
            "lifecycle_msgs/srv/ChangeState" =>
            new_untyped_service_support_lifecycle_msgs_srv_ChangeState,
            "lifecycle_msgs/srv/GetAvailableStates" =>
            new_untyped_service_support_lifecycle_msgs_srv_GetAvailableStates,
            "lifecycle_msgs/srv/GetAvailableTransitions" =>
            new_untyped_service_support_lifecycle_msgs_srv_GetAvailableTransitions,
            "lifecycle_msgs/srv/GetState" =>
            new_untyped_service_support_lifecycle_msgs_srv_GetState,
            "nav_msgs/srv/GetMap" => new_untyped_service_support_nav_msgs_srv_GetMap,
            "nav_msgs/srv/GetPlan" => new_untyped_service_support_nav_msgs_srv_GetPlan,
            "nav_msgs/srv/LoadMap" => new_untyped_service_support_nav_msgs_srv_LoadMap,
            "nav_msgs/srv/SetMap" => new_untyped_service_support_nav_msgs_srv_SetMap,
            "rcl_interfaces/srv/DescribeParameters" =>
            new_untyped_service_support_rcl_interfaces_srv_DescribeParameters,
            "rcl_interfaces/srv/GetParameterTypes" =>
            new_untyped_service_support_rcl_interfaces_srv_GetParameterTypes,
            "rcl_interfaces/srv/GetParameters" =>
            new_untyped_service_support_rcl_interfaces_srv_GetParameters,
            "rcl_interfaces/srv/ListParameters" =>
            new_untyped_service_support_rcl_interfaces_srv_ListParameters,
            "rcl_interfaces/srv/SetParameters" =>
            new_untyped_service_support_rcl_interfaces_srv_SetParameters,
            "rcl_interfaces/srv/SetParametersAtomically" =>
            new_untyped_service_support_rcl_interfaces_srv_SetParametersAtomically,
            "rosbag2_interfaces/srv/Burst" =>
            new_untyped_service_support_rosbag2_interfaces_srv_Burst,
            "rosbag2_interfaces/srv/GetRate" =>
            new_untyped_service_support_rosbag2_interfaces_srv_GetRate,
            "rosbag2_interfaces/srv/IsPaused" =>
            new_untyped_service_support_rosbag2_interfaces_srv_IsPaused,
            "rosbag2_interfaces/srv/Pause" =>
            new_untyped_service_support_rosbag2_interfaces_srv_Pause,
            "rosbag2_interfaces/srv/PlayNext" =>
            new_untyped_service_support_rosbag2_interfaces_srv_PlayNext,
            "rosbag2_interfaces/srv/Resume" =>
            new_untyped_service_support_rosbag2_interfaces_srv_Resume,
            "rosbag2_interfaces/srv/Seek" =>
            new_untyped_service_support_rosbag2_interfaces_srv_Seek,
            "rosbag2_interfaces/srv/SetRate" =>
            new_untyped_service_support_rosbag2_interfaces_srv_SetRate,
            "rosbag2_interfaces/srv/Snapshot" =>
            new_untyped_service_support_rosbag2_interfaces_srv_Snapshot,
            "rosbag2_interfaces/srv/TogglePaused" =>
            new_untyped_service_support_rosbag2_interfaces_srv_TogglePaused,
            "sensor_msgs/srv/SetCameraInfo" =>
            new_untyped_service_support_sensor_msgs_srv_SetCameraInfo,
            "std_srvs/srv/Empty" => new_untyped_service_support_std_srvs_srv_Empty,
            "std_srvs/srv/SetBool" => new_untyped_service_support_std_srvs_srv_SetBool,
            "std_srvs/srv/Trigger" => new_untyped_service_support_std_srvs_srv_Trigger,
            "tf2_msgs/srv/FrameGraph" =>
            new_untyped_service_support_tf2_msgs_srv_FrameGraph,
            "visualization_msgs/srv/GetInteractiveMarkers" =>
            new_untyped_service_support_visualization_msgs_srv_GetInteractiveMarkers
        };
        let func = MAP
            .get(typename)
            .ok_or_else(|| Error::InvalidMessageType {
                msgtype: typename.into(),
            })?;
        Ok(func())
    }
}
