Download Youtube Video in Rust
==================

Inspired
---------------
- [https://github.com/lepidosteus/youtube-dl](https://github.com/lepidosteus/youtube-dl)
- [https://github.com/kkdai/youtube](https://github.com/kkdai/youtube)

How it works
---------------

- Parse the video ID you input in URL
	- ex: `https://www.youtube.com/watch?v=kLg5Ghv0gGE`, the video id is `kLg5Ghv0gGE`
- Get video information via video id.
	- Use URL: `http://youtube.com/get_video_info?video_id=`
- Parse and decode video information.
	- Download URL in "url="
	- title in "title="
- Download video from URL
	- Need the string combination of "url+sig"

Usage
---------------
-  cargo run https://www.youtube.com/watch?v=kLg5Ghv0gGE