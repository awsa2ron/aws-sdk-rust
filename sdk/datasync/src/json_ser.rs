// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_cancel_task_execution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelTaskExecutionInput,
) {
    if let Some(var_1) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_1);
    }
}

pub fn serialize_structure_create_agent_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAgentInput,
) {
    if let Some(var_2) = &input.activation_key {
        object.key("ActivationKey").string(var_2);
    }
    if let Some(var_3) = &input.agent_name {
        object.key("AgentName").string(var_3);
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_7, item_6);
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_8);
    }
    if let Some(var_9) = &input.subnet_arns {
        let mut array_10 = object.key("SubnetArns").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11);
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.security_group_arns {
        let mut array_13 = object.key("SecurityGroupArns").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14);
            }
        }
        array_13.finish();
    }
}

pub fn serialize_structure_create_location_efs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationEfsInput,
) {
    if let Some(var_15) = &input.subdirectory {
        object.key("Subdirectory").string(var_15);
    }
    if let Some(var_16) = &input.efs_filesystem_arn {
        object.key("EfsFilesystemArn").string(var_16);
    }
    if let Some(var_17) = &input.ec2_config {
        let mut object_18 = object.key("Ec2Config").start_object();
        crate::json_ser::serialize_structure_ec2_config(&mut object_18, var_17);
        object_18.finish();
    }
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("Tags").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_22, item_21);
                object_22.finish();
            }
        }
        array_20.finish();
    }
}

pub fn serialize_structure_create_location_fsx_windows_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationFsxWindowsInput,
) {
    if let Some(var_23) = &input.subdirectory {
        object.key("Subdirectory").string(var_23);
    }
    if let Some(var_24) = &input.fsx_filesystem_arn {
        object.key("FsxFilesystemArn").string(var_24);
    }
    if let Some(var_25) = &input.security_group_arns {
        let mut array_26 = object.key("SecurityGroupArns").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27);
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.tags {
        let mut array_29 = object.key("Tags").start_array();
        for item_30 in var_28 {
            {
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_31, item_30);
                object_31.finish();
            }
        }
        array_29.finish();
    }
    if let Some(var_32) = &input.user {
        object.key("User").string(var_32);
    }
    if let Some(var_33) = &input.domain {
        object.key("Domain").string(var_33);
    }
    if let Some(var_34) = &input.password {
        object.key("Password").string(var_34);
    }
}

pub fn serialize_structure_create_location_nfs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationNfsInput,
) {
    if let Some(var_35) = &input.subdirectory {
        object.key("Subdirectory").string(var_35);
    }
    if let Some(var_36) = &input.server_hostname {
        object.key("ServerHostname").string(var_36);
    }
    if let Some(var_37) = &input.on_prem_config {
        let mut object_38 = object.key("OnPremConfig").start_object();
        crate::json_ser::serialize_structure_on_prem_config(&mut object_38, var_37);
        object_38.finish();
    }
    if let Some(var_39) = &input.mount_options {
        let mut object_40 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_nfs_mount_options(&mut object_40, var_39);
        object_40.finish();
    }
    if let Some(var_41) = &input.tags {
        let mut array_42 = object.key("Tags").start_array();
        for item_43 in var_41 {
            {
                let mut object_44 = array_42.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_44, item_43);
                object_44.finish();
            }
        }
        array_42.finish();
    }
}

pub fn serialize_structure_create_location_object_storage_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationObjectStorageInput,
) {
    if let Some(var_45) = &input.server_hostname {
        object.key("ServerHostname").string(var_45);
    }
    if let Some(var_46) = &input.server_port {
        object.key("ServerPort").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_46).into()),
        );
    }
    if let Some(var_47) = &input.server_protocol {
        object.key("ServerProtocol").string(var_47.as_str());
    }
    if let Some(var_48) = &input.subdirectory {
        object.key("Subdirectory").string(var_48);
    }
    if let Some(var_49) = &input.bucket_name {
        object.key("BucketName").string(var_49);
    }
    if let Some(var_50) = &input.access_key {
        object.key("AccessKey").string(var_50);
    }
    if let Some(var_51) = &input.secret_key {
        object.key("SecretKey").string(var_51);
    }
    if let Some(var_52) = &input.agent_arns {
        let mut array_53 = object.key("AgentArns").start_array();
        for item_54 in var_52 {
            {
                array_53.value().string(item_54);
            }
        }
        array_53.finish();
    }
    if let Some(var_55) = &input.tags {
        let mut array_56 = object.key("Tags").start_array();
        for item_57 in var_55 {
            {
                let mut object_58 = array_56.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_58, item_57);
                object_58.finish();
            }
        }
        array_56.finish();
    }
}

pub fn serialize_structure_create_location_s3_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationS3Input,
) {
    if let Some(var_59) = &input.subdirectory {
        object.key("Subdirectory").string(var_59);
    }
    if let Some(var_60) = &input.s3_bucket_arn {
        object.key("S3BucketArn").string(var_60);
    }
    if let Some(var_61) = &input.s3_storage_class {
        object.key("S3StorageClass").string(var_61.as_str());
    }
    if let Some(var_62) = &input.s3_config {
        let mut object_63 = object.key("S3Config").start_object();
        crate::json_ser::serialize_structure_s3_config(&mut object_63, var_62);
        object_63.finish();
    }
    if let Some(var_64) = &input.agent_arns {
        let mut array_65 = object.key("AgentArns").start_array();
        for item_66 in var_64 {
            {
                array_65.value().string(item_66);
            }
        }
        array_65.finish();
    }
    if let Some(var_67) = &input.tags {
        let mut array_68 = object.key("Tags").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_70, item_69);
                object_70.finish();
            }
        }
        array_68.finish();
    }
}

pub fn serialize_structure_create_location_smb_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLocationSmbInput,
) {
    if let Some(var_71) = &input.subdirectory {
        object.key("Subdirectory").string(var_71);
    }
    if let Some(var_72) = &input.server_hostname {
        object.key("ServerHostname").string(var_72);
    }
    if let Some(var_73) = &input.user {
        object.key("User").string(var_73);
    }
    if let Some(var_74) = &input.domain {
        object.key("Domain").string(var_74);
    }
    if let Some(var_75) = &input.password {
        object.key("Password").string(var_75);
    }
    if let Some(var_76) = &input.agent_arns {
        let mut array_77 = object.key("AgentArns").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78);
            }
        }
        array_77.finish();
    }
    if let Some(var_79) = &input.mount_options {
        let mut object_80 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_smb_mount_options(&mut object_80, var_79);
        object_80.finish();
    }
    if let Some(var_81) = &input.tags {
        let mut array_82 = object.key("Tags").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_84, item_83);
                object_84.finish();
            }
        }
        array_82.finish();
    }
}

pub fn serialize_structure_create_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTaskInput,
) {
    if let Some(var_85) = &input.source_location_arn {
        object.key("SourceLocationArn").string(var_85);
    }
    if let Some(var_86) = &input.destination_location_arn {
        object.key("DestinationLocationArn").string(var_86);
    }
    if let Some(var_87) = &input.cloud_watch_log_group_arn {
        object.key("CloudWatchLogGroupArn").string(var_87);
    }
    if let Some(var_88) = &input.name {
        object.key("Name").string(var_88);
    }
    if let Some(var_89) = &input.options {
        let mut object_90 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_options(&mut object_90, var_89);
        object_90.finish();
    }
    if let Some(var_91) = &input.excludes {
        let mut array_92 = object.key("Excludes").start_array();
        for item_93 in var_91 {
            {
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_filter_rule(&mut object_94, item_93);
                object_94.finish();
            }
        }
        array_92.finish();
    }
    if let Some(var_95) = &input.schedule {
        let mut object_96 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_task_schedule(&mut object_96, var_95);
        object_96.finish();
    }
    if let Some(var_97) = &input.tags {
        let mut array_98 = object.key("Tags").start_array();
        for item_99 in var_97 {
            {
                let mut object_100 = array_98.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_100, item_99);
                object_100.finish();
            }
        }
        array_98.finish();
    }
}

pub fn serialize_structure_delete_agent_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAgentInput,
) {
    if let Some(var_101) = &input.agent_arn {
        object.key("AgentArn").string(var_101);
    }
}

pub fn serialize_structure_delete_location_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLocationInput,
) {
    if let Some(var_102) = &input.location_arn {
        object.key("LocationArn").string(var_102);
    }
}

pub fn serialize_structure_delete_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTaskInput,
) {
    if let Some(var_103) = &input.task_arn {
        object.key("TaskArn").string(var_103);
    }
}

pub fn serialize_structure_describe_agent_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAgentInput,
) {
    if let Some(var_104) = &input.agent_arn {
        object.key("AgentArn").string(var_104);
    }
}

pub fn serialize_structure_describe_location_efs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationEfsInput,
) {
    if let Some(var_105) = &input.location_arn {
        object.key("LocationArn").string(var_105);
    }
}

pub fn serialize_structure_describe_location_fsx_windows_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationFsxWindowsInput,
) {
    if let Some(var_106) = &input.location_arn {
        object.key("LocationArn").string(var_106);
    }
}

pub fn serialize_structure_describe_location_nfs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationNfsInput,
) {
    if let Some(var_107) = &input.location_arn {
        object.key("LocationArn").string(var_107);
    }
}

pub fn serialize_structure_describe_location_object_storage_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationObjectStorageInput,
) {
    if let Some(var_108) = &input.location_arn {
        object.key("LocationArn").string(var_108);
    }
}

pub fn serialize_structure_describe_location_s3_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationS3Input,
) {
    if let Some(var_109) = &input.location_arn {
        object.key("LocationArn").string(var_109);
    }
}

pub fn serialize_structure_describe_location_smb_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLocationSmbInput,
) {
    if let Some(var_110) = &input.location_arn {
        object.key("LocationArn").string(var_110);
    }
}

pub fn serialize_structure_describe_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTaskInput,
) {
    if let Some(var_111) = &input.task_arn {
        object.key("TaskArn").string(var_111);
    }
}

pub fn serialize_structure_describe_task_execution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTaskExecutionInput,
) {
    if let Some(var_112) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_112);
    }
}

pub fn serialize_structure_list_agents_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAgentsInput,
) {
    if let Some(var_113) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_113).into()),
        );
    }
    if let Some(var_114) = &input.next_token {
        object.key("NextToken").string(var_114);
    }
}

pub fn serialize_structure_list_locations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLocationsInput,
) {
    if let Some(var_115) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_115).into()),
        );
    }
    if let Some(var_116) = &input.next_token {
        object.key("NextToken").string(var_116);
    }
    if let Some(var_117) = &input.filters {
        let mut array_118 = object.key("Filters").start_array();
        for item_119 in var_117 {
            {
                let mut object_120 = array_118.value().start_object();
                crate::json_ser::serialize_structure_location_filter(&mut object_120, item_119);
                object_120.finish();
            }
        }
        array_118.finish();
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_121) = &input.resource_arn {
        object.key("ResourceArn").string(var_121);
    }
    if let Some(var_122) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_122).into()),
        );
    }
    if let Some(var_123) = &input.next_token {
        object.key("NextToken").string(var_123);
    }
}

pub fn serialize_structure_list_task_executions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTaskExecutionsInput,
) {
    if let Some(var_124) = &input.task_arn {
        object.key("TaskArn").string(var_124);
    }
    if let Some(var_125) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_125).into()),
        );
    }
    if let Some(var_126) = &input.next_token {
        object.key("NextToken").string(var_126);
    }
}

pub fn serialize_structure_list_tasks_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTasksInput,
) {
    if let Some(var_127) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_127).into()),
        );
    }
    if let Some(var_128) = &input.next_token {
        object.key("NextToken").string(var_128);
    }
    if let Some(var_129) = &input.filters {
        let mut array_130 = object.key("Filters").start_array();
        for item_131 in var_129 {
            {
                let mut object_132 = array_130.value().start_object();
                crate::json_ser::serialize_structure_task_filter(&mut object_132, item_131);
                object_132.finish();
            }
        }
        array_130.finish();
    }
}

pub fn serialize_structure_start_task_execution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartTaskExecutionInput,
) {
    if let Some(var_133) = &input.task_arn {
        object.key("TaskArn").string(var_133);
    }
    if let Some(var_134) = &input.override_options {
        let mut object_135 = object.key("OverrideOptions").start_object();
        crate::json_ser::serialize_structure_options(&mut object_135, var_134);
        object_135.finish();
    }
    if let Some(var_136) = &input.includes {
        let mut array_137 = object.key("Includes").start_array();
        for item_138 in var_136 {
            {
                let mut object_139 = array_137.value().start_object();
                crate::json_ser::serialize_structure_filter_rule(&mut object_139, item_138);
                object_139.finish();
            }
        }
        array_137.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_140) = &input.resource_arn {
        object.key("ResourceArn").string(var_140);
    }
    if let Some(var_141) = &input.tags {
        let mut array_142 = object.key("Tags").start_array();
        for item_143 in var_141 {
            {
                let mut object_144 = array_142.value().start_object();
                crate::json_ser::serialize_structure_tag_list_entry(&mut object_144, item_143);
                object_144.finish();
            }
        }
        array_142.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_145) = &input.resource_arn {
        object.key("ResourceArn").string(var_145);
    }
    if let Some(var_146) = &input.keys {
        let mut array_147 = object.key("Keys").start_array();
        for item_148 in var_146 {
            {
                array_147.value().string(item_148);
            }
        }
        array_147.finish();
    }
}

pub fn serialize_structure_update_agent_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAgentInput,
) {
    if let Some(var_149) = &input.agent_arn {
        object.key("AgentArn").string(var_149);
    }
    if let Some(var_150) = &input.name {
        object.key("Name").string(var_150);
    }
}

pub fn serialize_structure_update_location_nfs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationNfsInput,
) {
    if let Some(var_151) = &input.location_arn {
        object.key("LocationArn").string(var_151);
    }
    if let Some(var_152) = &input.subdirectory {
        object.key("Subdirectory").string(var_152);
    }
    if let Some(var_153) = &input.on_prem_config {
        let mut object_154 = object.key("OnPremConfig").start_object();
        crate::json_ser::serialize_structure_on_prem_config(&mut object_154, var_153);
        object_154.finish();
    }
    if let Some(var_155) = &input.mount_options {
        let mut object_156 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_nfs_mount_options(&mut object_156, var_155);
        object_156.finish();
    }
}

pub fn serialize_structure_update_location_object_storage_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationObjectStorageInput,
) {
    if let Some(var_157) = &input.location_arn {
        object.key("LocationArn").string(var_157);
    }
    if let Some(var_158) = &input.server_port {
        object.key("ServerPort").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_158).into()),
        );
    }
    if let Some(var_159) = &input.server_protocol {
        object.key("ServerProtocol").string(var_159.as_str());
    }
    if let Some(var_160) = &input.subdirectory {
        object.key("Subdirectory").string(var_160);
    }
    if let Some(var_161) = &input.access_key {
        object.key("AccessKey").string(var_161);
    }
    if let Some(var_162) = &input.secret_key {
        object.key("SecretKey").string(var_162);
    }
    if let Some(var_163) = &input.agent_arns {
        let mut array_164 = object.key("AgentArns").start_array();
        for item_165 in var_163 {
            {
                array_164.value().string(item_165);
            }
        }
        array_164.finish();
    }
}

pub fn serialize_structure_update_location_smb_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLocationSmbInput,
) {
    if let Some(var_166) = &input.location_arn {
        object.key("LocationArn").string(var_166);
    }
    if let Some(var_167) = &input.subdirectory {
        object.key("Subdirectory").string(var_167);
    }
    if let Some(var_168) = &input.user {
        object.key("User").string(var_168);
    }
    if let Some(var_169) = &input.domain {
        object.key("Domain").string(var_169);
    }
    if let Some(var_170) = &input.password {
        object.key("Password").string(var_170);
    }
    if let Some(var_171) = &input.agent_arns {
        let mut array_172 = object.key("AgentArns").start_array();
        for item_173 in var_171 {
            {
                array_172.value().string(item_173);
            }
        }
        array_172.finish();
    }
    if let Some(var_174) = &input.mount_options {
        let mut object_175 = object.key("MountOptions").start_object();
        crate::json_ser::serialize_structure_smb_mount_options(&mut object_175, var_174);
        object_175.finish();
    }
}

pub fn serialize_structure_update_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTaskInput,
) {
    if let Some(var_176) = &input.task_arn {
        object.key("TaskArn").string(var_176);
    }
    if let Some(var_177) = &input.options {
        let mut object_178 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_options(&mut object_178, var_177);
        object_178.finish();
    }
    if let Some(var_179) = &input.excludes {
        let mut array_180 = object.key("Excludes").start_array();
        for item_181 in var_179 {
            {
                let mut object_182 = array_180.value().start_object();
                crate::json_ser::serialize_structure_filter_rule(&mut object_182, item_181);
                object_182.finish();
            }
        }
        array_180.finish();
    }
    if let Some(var_183) = &input.schedule {
        let mut object_184 = object.key("Schedule").start_object();
        crate::json_ser::serialize_structure_task_schedule(&mut object_184, var_183);
        object_184.finish();
    }
    if let Some(var_185) = &input.name {
        object.key("Name").string(var_185);
    }
    if let Some(var_186) = &input.cloud_watch_log_group_arn {
        object.key("CloudWatchLogGroupArn").string(var_186);
    }
}

pub fn serialize_structure_update_task_execution_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTaskExecutionInput,
) {
    if let Some(var_187) = &input.task_execution_arn {
        object.key("TaskExecutionArn").string(var_187);
    }
    if let Some(var_188) = &input.options {
        let mut object_189 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_options(&mut object_189, var_188);
        object_189.finish();
    }
}

pub fn serialize_structure_tag_list_entry(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagListEntry,
) {
    if let Some(var_190) = &input.key {
        object.key("Key").string(var_190);
    }
    if let Some(var_191) = &input.value {
        object.key("Value").string(var_191);
    }
}

pub fn serialize_structure_ec2_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Ec2Config,
) {
    if let Some(var_192) = &input.subnet_arn {
        object.key("SubnetArn").string(var_192);
    }
    if let Some(var_193) = &input.security_group_arns {
        let mut array_194 = object.key("SecurityGroupArns").start_array();
        for item_195 in var_193 {
            {
                array_194.value().string(item_195);
            }
        }
        array_194.finish();
    }
}

pub fn serialize_structure_on_prem_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnPremConfig,
) {
    if let Some(var_196) = &input.agent_arns {
        let mut array_197 = object.key("AgentArns").start_array();
        for item_198 in var_196 {
            {
                array_197.value().string(item_198);
            }
        }
        array_197.finish();
    }
}

pub fn serialize_structure_nfs_mount_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NfsMountOptions,
) {
    if let Some(var_199) = &input.version {
        object.key("Version").string(var_199.as_str());
    }
}

pub fn serialize_structure_s3_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Config,
) {
    if let Some(var_200) = &input.bucket_access_role_arn {
        object.key("BucketAccessRoleArn").string(var_200);
    }
}

pub fn serialize_structure_smb_mount_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SmbMountOptions,
) {
    if let Some(var_201) = &input.version {
        object.key("Version").string(var_201.as_str());
    }
}

pub fn serialize_structure_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Options,
) {
    if let Some(var_202) = &input.verify_mode {
        object.key("VerifyMode").string(var_202.as_str());
    }
    if let Some(var_203) = &input.overwrite_mode {
        object.key("OverwriteMode").string(var_203.as_str());
    }
    if let Some(var_204) = &input.atime {
        object.key("Atime").string(var_204.as_str());
    }
    if let Some(var_205) = &input.mtime {
        object.key("Mtime").string(var_205.as_str());
    }
    if let Some(var_206) = &input.uid {
        object.key("Uid").string(var_206.as_str());
    }
    if let Some(var_207) = &input.gid {
        object.key("Gid").string(var_207.as_str());
    }
    if let Some(var_208) = &input.preserve_deleted_files {
        object.key("PreserveDeletedFiles").string(var_208.as_str());
    }
    if let Some(var_209) = &input.preserve_devices {
        object.key("PreserveDevices").string(var_209.as_str());
    }
    if let Some(var_210) = &input.posix_permissions {
        object.key("PosixPermissions").string(var_210.as_str());
    }
    if let Some(var_211) = &input.bytes_per_second {
        object.key("BytesPerSecond").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_211).into()),
        );
    }
    if let Some(var_212) = &input.task_queueing {
        object.key("TaskQueueing").string(var_212.as_str());
    }
    if let Some(var_213) = &input.log_level {
        object.key("LogLevel").string(var_213.as_str());
    }
    if let Some(var_214) = &input.transfer_mode {
        object.key("TransferMode").string(var_214.as_str());
    }
    if let Some(var_215) = &input.security_descriptor_copy_flags {
        object
            .key("SecurityDescriptorCopyFlags")
            .string(var_215.as_str());
    }
}

pub fn serialize_structure_filter_rule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FilterRule,
) {
    if let Some(var_216) = &input.filter_type {
        object.key("FilterType").string(var_216.as_str());
    }
    if let Some(var_217) = &input.value {
        object.key("Value").string(var_217);
    }
}

pub fn serialize_structure_task_schedule(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TaskSchedule,
) {
    if let Some(var_218) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_218);
    }
}

pub fn serialize_structure_location_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LocationFilter,
) {
    if let Some(var_219) = &input.name {
        object.key("Name").string(var_219.as_str());
    }
    if let Some(var_220) = &input.values {
        let mut array_221 = object.key("Values").start_array();
        for item_222 in var_220 {
            {
                array_221.value().string(item_222);
            }
        }
        array_221.finish();
    }
    if let Some(var_223) = &input.operator {
        object.key("Operator").string(var_223.as_str());
    }
}

pub fn serialize_structure_task_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TaskFilter,
) {
    if let Some(var_224) = &input.name {
        object.key("Name").string(var_224.as_str());
    }
    if let Some(var_225) = &input.values {
        let mut array_226 = object.key("Values").start_array();
        for item_227 in var_225 {
            {
                array_226.value().string(item_227);
            }
        }
        array_226.finish();
    }
    if let Some(var_228) = &input.operator {
        object.key("Operator").string(var_228.as_str());
    }
}