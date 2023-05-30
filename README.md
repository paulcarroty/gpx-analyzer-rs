# rs-gpx-analyzer  [![BUild Status](https://github.com/paulcarroty/rs-gpx-analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/paulcarroty/rs-gpx-analyzer/actions)


Find the best GPS tracks in GPX (GPS Exchange Format)

You can export GPS tracks from apps like Strava or Nike Run Club and rs-gpx-analyzer will highlight all the top tracks with coordinates.

### Installation

Install Rust and Cargo, and then run:


```
$ git clone https://github.com/paulcarroty/rs-gpx-analyzer.git && cd rs-gpx-analyzer
cargo install 
```

Alternative way: download [CI artifact](https://github.com/paulcarroty/rs-gpx-analyzer/actions/) for your OS.



### How to use

`$ rs-gpx-analyzer /path/to/file.gpx`


### Example 

```
rust-gpx-analyzer data/tour.gpx
Loading GPX trek with size 162656 bytes...
The track name is: 1_Roscoff_Morlaix_A
Number of segments in loaded track: 1
Waypoints in segment 0 : 1741
New max elevation: 5.30000000000291 on distance: 0.0 at [-3.982993563773938, 48.726304979176675]
New max distance: 8.604 on elevation: 4.69999999999709 at [-3.9829726446543385, 48.72623035828412]
New max distance: 11.685 on elevation: 5.19999999999709 at [-3.9829546542797467, 48.72612667110164]
New max distance: 18.696 on elevation: 5.69999999999709 at [-3.9829070729298808, 48.725965124843256]
...
--------------------The-best-tracks-------------------

Highest elevation: 9999.0 at [-3.9857610894015805, 48.68644800612525]
Longest distance: 155.452 at [-3.9577466263357945, 48.64466328878896] on elevation 0.0
```
 
