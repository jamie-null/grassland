
mod suppression {
    /* 

    SUPPRESSION OF NON-SALIENT IMAGE REGIONS
    https://www.quantamagazine.org/your-brain-chooses-what-to-let-you-see-20190930/
    https://www.quantamagazine.org/to-pay-attention-the-brain-uses-filters-not-a-spotlight-20190924/

    In order to not waste GPU power, once the node has established the positions of the background, static objects like buildings etc. it can ignore them for the most part. The majority of a FOV will often be the sky and sides of buildings in which not many changes will be occurring. So unless the motion detector sees movement there, we can cut the image (or just white or black out those regions of the image so the region proposal system will quickly overlook them. That needs to be tested) to the regions where there is movement so we send a smaller image to the neural network. We can choose to scan the whole image every four seconds or so.

    */
}
  
pub mod detection {
    // Use Coco or cityscapes for detection. It seems to generalize well

    // Keep in mind there are other data sets like nuscenes and apolloscape (which has a lot)


    mod pose_detection {

        // OpenPose isn't that good sometimes -> https://www.youtube.com/watch?v=OgCyBjqXDLs
        // POSE DETECTION
        // https://nanonets.com/blog/human-pose-estimation-2d-guide/
        // PoseNet has a good license https://github.com/tensorflow/tfjs-models/tree/master/posenet

    }

    mod depth {
        /* Use Kitti dataset for depth. It's very good and fast. Kitti detection doesn't seem to generalize as well as Coco. Since Coco is a bigger dataset.

        Monodepth uses Kitti (https://www.youtube.com/watch?v=5nCQ59YF0h4). But monodepth's license won't allow non commercial. 

        Use this -> https://www.youtube.com/watch?v=LAIo9z_0KIk ( https://github.com/zhengqili/MegaDepth). Megadepth doesn't use Kitti though.

        No. I think you should use this -> https://github.com/YvanYin/VNL_Monocular_Depth_Prediction if you can get the license

          */

    }

    mod direction {
        // You don't know which way the vehicle is headed. A cheap way is optical flow
    }

    mod tracking {
        // If I can run the frames fast enough using suppression, it won't matter at least for the demo
    }


    pub mod hash {
        pub fn hash_each_activation() {}
        pub fn hash_activation(numpy_array_bytes: Vec<i32>) {}
        pub fn hash_output(nn_output: Vec<i32>) {}

    }



    pub mod server {
        use std::thread;
        use std::sync::mpsc;
        use std::net::{TcpListener, TcpStream, Shutdown};
        use std::io::{self, BufReader, Write};
        use crate::settings::DetectionServerMeta;

        pub fn connect(detection_server_meta: &DetectionServerMeta) -> TcpStream {

            let mut stream = TcpStream::connect(format!("{}:{}", detection_server_meta.ip, detection_server_meta.port)).unwrap();

            // send magic_bytes to identify yourself to server

            const MAGIC_BYTES: [u8; 4] = [0xf9, 0xbe, 0xb5, 0xd9];
            stream.write(&MAGIC_BYTES).unwrap();
            stream.flush().unwrap();
            println!("sent magic bytes to detection server");
            
            // let reader = BufReader::new(stream);

            return stream;
        }

    }
}
