cfg_if::cfg_if! {
    if #[cfg(not(feature="use_ros_shim"))] {
        use std::env;

        const AMENT_PREFIX_PATH: &str = "AMENT_PREFIX_PATH";

        fn get_env_var_or_abort(env_var: &'static str) -> String {
            if let Ok(value) = env::var(env_var) {
                value
            } else {
                panic!(
                    "{} environment variable not set - please source ROS 2 installation first.",
                    env_var
                );
            }
        }
    }
}

// Gate the primitive-sequence layout on the ROS distro, like other distro
// differences in rclrs (starting with Lyrical, primitive sequences carry extra
// ABI flags; see `Sequence`/`BufferFlags`).
const ROS_DISTRO: &str = "ROS_DISTRO";
const KNOWN_DISTROS: &[&str] = &["humble", "jazzy", "kilted", "lyrical", "rolling"];

fn get_ros_distro() -> String {
    std::env::var(ROS_DISTRO)
        .or_else(|_| {
            if std::env::var("CARGO_FEATURE_USE_ROS_SHIM").is_ok() {
                rustflags::from_env()
                    .find_map(|f| match f {
                        rustflags::Flag::Cfg { name, value } if name.as_str() == "ros_distro" => {
                            value
                        }
                        _ => None,
                    })
                    .ok_or_else(|| "Missing --cfg ros_distro in RUSTFLAGS".to_string())
            } else {
                Err(format!("Set {ROS_DISTRO} or use ROS shim"))
            }
        })
        .expect("Failed to determine ROS distro")
}

fn main() {
    println!(
        "cargo:rustc-check-cfg=cfg(ros_distro, values(\"{}\"))",
        KNOWN_DISTROS.join("\", \"")
    );
    println!("cargo:rustc-cfg=ros_distro=\"{}\"", get_ros_distro());
    println!("cargo:rerun-if-env-changed={ROS_DISTRO}");

    #[cfg(not(feature = "use_ros_shim"))]
    {
        let ament_prefix_path_list = get_env_var_or_abort(AMENT_PREFIX_PATH);
        for ament_prefix_path in env::split_paths(&ament_prefix_path_list) {
            let library_path = ament_prefix_path.join("lib");
            println!("cargo:rustc-link-search=native={}", library_path.display());
        }
    }

    // Invalidate the built crate whenever this script changes
    println!("cargo:rerun-if-changed=build.rs");
}
