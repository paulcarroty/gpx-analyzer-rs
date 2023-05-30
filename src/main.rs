use geoutils::Location;
use gpx::{read, Gpx, Track, TrackSegment};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please enter a path to .gpx file");
        process::exit(0x0100);
    }
    let file_path = &args[1];
    let file = File::open(file_path).expect("Should have been able to read the file");
    let filesize = file.metadata().unwrap().len();
    println!("Loading GPX trek with size {} bytes...", filesize);
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    let track_name = &track.name;

    println!("The track name is: {}", track_name.as_ref().unwrap());

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];

    println!(
        "Number of segments in loaded track: {:?}",
        track.segments.len()
    );

    // print number of waiypoints in each segment
    for sgmt in 0..track.segments.len() {
        //for (index, segment) in &track.segments {
        println!(
            "Waypoints in segment {sgmt} : {}",
            track.segments[sgmt].points.len() //track.segments[index].points.len()
        );
    }

    // Print elevation, latitude and longtitude for first 10 waypoins
    let mut max_elevation = 0.0;
    let mut max_elevation_lat = 0.0;
    let mut max_elevation_long = 0.0;

    let mut max_distance = 0.0;
    let mut max_distance_lat = 0.0;
    let mut max_distance_long = 0.0;
    let mut max_distance_elevation = 0.0;

    let mut old_latitude = segment.points[0].point().x();
    let mut old_longitude = segment.points[0].point().y();

    for wp in 0..track.segments[0].points.len() {
        let latitude = segment.points[wp].point().x();
        let longitude = segment.points[wp].point().y();
        let elevation = segment.points[wp].elevation.unwrap();

        // calculate distance in meters
        let previous_location = Location::new(old_latitude, old_longitude);
        let current_location = Location::new(latitude, longitude);
        let distance = current_location
            .distance_to(&previous_location)
            .unwrap()
            .meters();

        if distance > max_distance {
            max_distance = distance;
            max_distance_lat = latitude;
            max_distance_long = longitude;
            max_distance_elevation = elevation;
            println!(
                "{}{:?}{}{:?}{}{:?}",
                "New max distance: ",
                distance,
                " on elevation: ",
                elevation,
                " at ",
                [max_distance_lat, max_distance_long]
            );
        }
        old_latitude = latitude;
        old_longitude = longitude;

        if elevation > max_elevation {
            max_elevation = elevation;
            max_elevation_lat = latitude;
            max_elevation_long = longitude;
            println!(
                "{}{:?}{}{:?}{}{:?}",
                "New max elevation: ",
                elevation,
                " on distance: ",
                distance,
                " at ",
                [max_elevation_lat, max_elevation_long]
            );
        }
    }

    println!("--------------------The-best-tracks-------------------\n");
    println!(
        "{}{:?}{}{:?}",
        "Highest elevation: ",
        max_elevation,
        " at ",
        [max_elevation_lat, max_elevation_long]
    );
    println!(
        "{}{:?}{}{:?}{}{:?}",
        "Longest distance: ",
        max_distance,
        " at ",
        [max_distance_lat, max_distance_long],
        " on elevation ",
        max_distance_elevation
    )
}
