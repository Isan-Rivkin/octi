pub const CMAKE_TARGET_LIBARY_TEMPLATE: &str = r#"
    ADD_LIBRARY(#OCTI_PROJECT_NAME# 
        #OCTI_PROJECT_FILES#
    )
    TARGET_INCLUDE_DIRECTORIES(#OCTI_PROJECT_NAME#
        #OCTI_PROJECT_INCLUDE_DIRS#
    )
    TARGET_LINK_LIBRARIES(#OCTI_PROJECT_NAME#
        #OCTI_LIB_TYPE#
        #OCTI_PROJECT_DEPENDENCIES#
    )
"#;

pub const CMAKE_TARGET_BINARY_TEMPLATE: &str = r#"
    ADD_EXECUTABLE(#OCTI_PROJECT_NAME# 
        #OCTI_PROJECT_FILES#
    )
    TARGET_INCLUDE_DIRECTORIES(#OCTI_PROJECT_NAME#
        #OCTI_PROJECT_INCLUDE_DIRS#
    )
    TARGET_LINK_LIBRARIES(#OCTI_PROJECT_NAME#
        #OCTI_PROJECT_DEPENDENCIES#
    )
"#;

pub const CMAKE_COMPILER_FLAGS_TEMPLATE: &str = r#"
    ADD_COMPILE_OPTIONS(#OCTI_COMPILER_FLAGS#)
"#;

pub const CMAKE_PACKAGE_METADATA_TEMPLATE: &str = r#"
    CMAKE_MINIMUM_REQUIRED(VERSION 3.2)
    PROJECT(#OCTI_PROJECT_NAME# VERSION #OCTI_PROJECT_VERSION#)
"#;

pub const CMAKE_TEST_TEMPLATE: &str = r#"
"#;

pub const CMAKE_CONFIG_TEMPLATE: &str = r#"
    CONFIGURE_FILE(#OCTI_FILE_INPUT# #OCTI_FILE_OUTPUT#)
"#;

pub const CPP_CONFIG_TEMPLATE: &str = r#"
#ifndef @PROJECT_NAME@_CONFIG_
#define @PROJECT_NAME@_CONFIG_

class @PROJECT_NAME@Info
{
public:
    static int major_version() { return @PROJECT_MAJOR_VERSION@; }
    static int minor_version() { return @PROJECT_MINOR_VERSION@; }
    static int patch_version() { return @PROJECT_PATCH_VERSION@; }
    static char * patch() { return "@PROJECT_VERSION@"; }
    static char * name() { return "@PROJECT_NAME@"; }
};

#endif
"#;
