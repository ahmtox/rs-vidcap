mod transcoding;
mod optimization;
mod captions;

fn main() {
    println!("Video processing started!");

    transcoding::transcoder::transcode_video("input.mp4", "output.mp4");
    optimization::optimizer::optimize_video("output.mp4");
    captions::captioner::generate_captions("output.mp4");
}