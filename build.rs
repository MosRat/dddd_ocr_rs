fn main(){
    #[cfg(feature = "use-ncnn")]
    {
        use std::env;
        // 设置环境变量
        env::set_var("NCNN_INCLUDE_DIR", "E:\\Software\\Nccn\\ncnn-20240820-windows-vs2022\\x64\\include\\ncnn");

        // 打印以验证设置
        println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
        println!("NCNN_INCLUDE_DIR is set to: {}", env::var("NCNN_INCLUDE_DIR").unwrap());



        // println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=static=msvcrt");
        println!("cargo:rustc-link-search=native=E:/Software/Nccn/ncnn-20240820-windows-vs2022/x64/lib");
        println!("cargo:rustc-link-lib=static=ncnn");
        println!("cargo:rustc-link-lib=static=GenericCodeGen");
        // println!("cargo:rustc-link-lib=static=glslang");
        println!("cargo:rustc-link-lib=static=MachineIndependent"); // 必须
        println!("cargo:rustc-link-lib=static=OSDependent"); // 必须
        println!("cargo:rustc-link-lib=static=SPIRV"); // 必须

        #[cfg(feature = "gpu")]
        {
            let vulkan_sdk = env::var("VULKAN_SDK").unwrap_or(r#"D:\Scoop\apps\vulkan\current"#.to_string());

            // 添加 Vulkan 库路径
            println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk);

            // 链接 Vulkan 库
            println!("cargo:rustc-link-lib=dylib=vulkan-1");
        }


        // 如果要开vulkan，添加vulkan的相关库
        // println!("cargo:rustc-link-lib=static=ncnn"); // static代表链接静态库libncnn.a或ncnn.lib
        // println!("cargo:rustc-link-lib=dylib=ncnn"); // static代表链接动态库libncnn.dylib或ncnn.dll
    }
}