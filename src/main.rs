use r2r;
use r2r::trajectory_msgs::msg::{JointTrajectory , JointTrajectoryPoint};
use r2r::sensor_msgs::msg::JointState;
use r2r::builtin_interfaces::msg::Duration;

use r2r::QosProfile;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "testnode", "")?;

    let mut sub = node.subscribe::<JointState>("/joint_states",
                                               QosProfile::default()).unwrap();

    let topic = format!("/ceiling_robot_controller/joint_trajectory");
    let p = node.create_publisher::<JointTrajectory>(&topic, QosProfile::default()).unwrap();

    let ceiling_joint_names = vec![
        "ceiling_shoulder_pan_joint".to_string(),
        "ceiling_shoulder_lift_joint".to_string(),
        "ceiling_elbow_joint".to_string(),
        "ceiling_wrist_1_joint".to_string(),
        "ceiling_wrist_2_joint".to_string(),
        "ceiling_wrist_3_joint".to_string(),
    ];

    tokio::task::spawn(async move {
        while let Some(msg) = sub.next().await {
            println!("got joint state msg");
            // wait a little until sending the command.
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            println!("Starting ceiling robot motion.");

            let mut jt = JointTrajectory::default();
            // jt.header = header;
            jt.joint_names = ceiling_joint_names.clone();
            let mut point = JointTrajectoryPoint::default();
            let mut pos_vec = vec![];
            for name in &ceiling_joint_names {
                if let Some(i) = msg.name.iter().position(|x| x==name) {
                    pos_vec.push(msg.position[i]);
                }
            }
            point.positions = pos_vec.clone();
            point.velocities = vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
            point.time_from_start = Duration {
                sec: 0,
                nanosec: 0,
            };
            jt.points.push(point);

            let mut point = JointTrajectoryPoint::default();
            point.positions = pos_vec.clone();
            point.velocities = vec![0.5, 0.5, -0.1, 0.0, 0.0, 0.0];
            point.time_from_start = Duration {
                sec: 4,
                nanosec: 0,
            };
            jt.points.push(point);

            let mut point = JointTrajectoryPoint::default();
            point.positions = pos_vec.clone();
            point.velocities = vec![0.0, 0.0, -1.0, 0.0, 0.0, 0.0];
            point.time_from_start = Duration {
                sec: 5,
                nanosec: 0,
            };
            jt.points.push(point);

            let mut point = JointTrajectoryPoint::default();
            point.positions = pos_vec.clone();
            point.velocities = vec![0.0, 0.0, -0.2, 0.0, 0.0, 0.0];
            point.time_from_start = Duration {
                sec: 8,
                nanosec: 0,
            };
            jt.points.push(point);

            let mut point = JointTrajectoryPoint::default();
            point.positions = pos_vec.clone();
            point.velocities = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
            point.time_from_start = Duration {
                sec: 10,
                nanosec: 0,
            };
            jt.points.push(point);

            // Send command
            let _ret = p.publish(&jt);

            // only once.
            break;
        }
    });


    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    handle.await?;

    Ok(())

}
