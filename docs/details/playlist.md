---
order: 10
icon: log
---

# Playlist Details

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Playlist details by ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/playlist?id=159144718
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/playlist?id=159144718' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Playlist details fetched successfully!",
	"data": {
		"id": "159144718",
		"name": "Feel Good Pop",
		"subtitle": "Just Updated",
		"type": "playlist",
		"url": "https://www.jiosaavn.com/featured/feel-good-pop/Er9SBlcxDtZFo9wdEAzFBA__",
		"image": "https://c.saavncdn.com/editorial/FeelGoodPop_20230804114615.jpg",
		"language": "",
		"year": 0,
		"playCount": 0,
		"explicitContent": false,
		"listCount": 30,
		"listType": "playlist",
		"userId": "phulki_user",
		"isDolbyContent": false,
		"lastUpdated": 1692000444,
		"username": "phulki_user",
		"firstname": "JioSaavn",
		"lastname": "",
		"followerCount": 14016,
		"fanCount": 13925,
		"share": 1,
		"videoCount": 0,
		"artists": [
			{
				"id": "579032",
				"name": "Jason Gill",
				"role": "music",
				"image": "https://c.saavncdn.com/810/Air-I-Breathe-English-2019-20190421030817-150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/jason-gill-songs/maSk,MLwtfE_"
			},
			{
				"id": "697092",
				"name": "Victor Rådström",
				"role": "music",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/victor-radstrom-songs/V8KrC2bFafs_"
			},
			{
				"id": "11879473",
				"name": "Mae Stephens",
				"role": "music",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
			},
			{
				"id": "696615",
				"name": "Meghan Trainor",
				"role": "music",
				"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
			},
			{
				"id": "2721559",
				"name": "Jake Torrey",
				"role": "music",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/jake-torrey-songs/HUTngr,8sN0_"
			},
			{
				"id": "11879473",
				"name": "Mae Stephens",
				"role": "singer",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
			},
			{
				"id": "696615",
				"name": "Meghan Trainor",
				"role": "singer",
				"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
			},
			{
				"id": "579032",
				"name": "Jason Gill",
				"role": "lyricist",
				"image": "https://c.saavncdn.com/810/Air-I-Breathe-English-2019-20190421030817-150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/jason-gill-songs/maSk,MLwtfE_"
			},
			{
				"id": "697092",
				"name": "Victor Rådström",
				"role": "lyricist",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/victor-radstrom-songs/V8KrC2bFafs_"
			},
			{
				"id": "11879473",
				"name": "Mae Stephens",
				"role": "lyricist",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
			},
			{
				"id": "696615",
				"name": "Meghan Trainor",
				"role": "lyricist",
				"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
			},
			{
				"id": "2721559",
				"name": "Jake Torrey",
				"role": "lyricist",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/jake-torrey-songs/HUTngr,8sN0_"
			},
			{
				"id": "577199",
				"name": "Harry Styles",
				"role": "music",
				"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
			},
			{
				"id": "658396",
				"name": "Kid Harpoon",
				"role": "music",
				"image": "",
				"type": "artist",
				"url": "https://www.jiosaavn.com/artist/kid-harpoon-songs/gI4xE2,DMko_"
			}
		],
		"subtitleDesc": ["1h 38m", "30 Songs", "13,925 Fans"],
		"songs": [
			{
				"id": "rNlUPw_l",
				"name": "Mr Right",
				"subtitle": "Mae Stephens, Meghan Trainor - Mr Right",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/mr-right/AiYHZCRHaF8",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/728/Mr-Right-English-2023-20230804063702-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/728/Mr-Right-English-2023-20230804063702-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/728/Mr-Right-English-2023-20230804063702-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 20272,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Jason Gill, Victor Rådström, Mae Stephens, Meghan Trainor, Jake Torrey",
				"song": null,
				"albumId": "47238807",
				"album": "Mr Right",
				"label": "EMI",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy7J+xae7NTJ00DreHn/tL2wEexs9eNn8TmwnQGlLAkLFZQi8tJ7aQNBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/mr-right/v3oSco8DTP8_",
				"duration": 167,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2023 Mae Stephens, under exclusive licence to Universal Music Operations Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "11879473",
							"name": "Mae Stephens",
							"role": "primary_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "579032",
							"name": "Jason Gill",
							"role": "music",
							"image": "https://c.saavncdn.com/810/Air-I-Breathe-English-2019-20190421030817-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jason-gill-songs/maSk,MLwtfE_"
						},
						{
							"id": "697092",
							"name": "Victor Rådström",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/victor-radstrom-songs/V8KrC2bFafs_"
						},
						{
							"id": "11879473",
							"name": "Mae Stephens",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "2721559",
							"name": "Jake Torrey",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jake-torrey-songs/HUTngr,8sN0_"
						},
						{
							"id": "11879473",
							"name": "Mae Stephens",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "579032",
							"name": "Jason Gill",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/810/Air-I-Breathe-English-2019-20190421030817-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jason-gill-songs/maSk,MLwtfE_"
						},
						{
							"id": "697092",
							"name": "Victor Rådström",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/victor-radstrom-songs/V8KrC2bFafs_"
						},
						{
							"id": "11879473",
							"name": "Mae Stephens",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mae-stephens-songs/huW6c3VEODs_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "2721559",
							"name": "Jake Torrey",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jake-torrey-songs/HUTngr,8sN0_"
						}
					]
				},
				"releaseDate": "2023-08-04",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "It96yG1r",
				"name": "As It Was",
				"subtitle": "Harry Styles - Harry&#039;s House",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/as-it-was/ORxSBw13BkE",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/061/Harry-s-House-English-2022-20220518224913-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/061/Harry-s-House-English-2022-20220518224913-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/061/Harry-s-House-English-2022-20220518224913-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2022,
				"playCount": 8532704,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Harry Styles, Kid Harpoon, Tyler Johnson",
				"song": null,
				"albumId": "35081244",
				"album": "Harry&#039;s House",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyafbGhocZIGEX7d4RweACgnR35n/+we01uAHyYcdfLt6G0e0zD5yAnBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/harrys-house/lM7QfO4kWRY_",
				"duration": 167,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "You know it's not the same as it was",
				"starred": false,
				"copyrightText": "(P) 2022 Erskine Records Limited, under exclusive license to Columbia Records, a Division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "658396",
							"name": "Kid Harpoon",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/kid-harpoon-songs/gI4xE2,DMko_"
						},
						{
							"id": "672872",
							"name": "Tyler Johnson",
							"role": "music",
							"image": "https://c.saavncdn.com/252/Grenade-Acoustic-Guitar--English-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-johnson-songs/std82FJ6ROw_"
						},
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "658396",
							"name": "Kid Harpoon",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/kid-harpoon-songs/gI4xE2,DMko_"
						},
						{
							"id": "672872",
							"name": "Tyler Johnson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/252/Grenade-Acoustic-Guitar--English-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-johnson-songs/std82FJ6ROw_"
						}
					]
				},
				"releaseDate": "2022-05-20",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "c-6-mB8N",
				"name": "Flowers",
				"subtitle": "Miley Cyrus - Flowers",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/flowers/E0VdHBlyD30",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/047/Flowers-English-2023-20230113044956-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/047/Flowers-English-2023-20230113044956-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/047/Flowers-English-2023-20230113044956-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 1350193,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Miley Cyrus, Michael Pollack, Gregory Aldae Hein",
				"song": null,
				"albumId": "41619705",
				"album": "Flowers",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyJmc6ckR1odnNeExbF9vWrq5h/Vz+00pCRQ4iiuPoyFZHLEgiNYOWZBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/flowers/PqoGNLQsMSA_",
				"duration": 200,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2023 Smiley Miley, Inc. under exclusive license to Columbia Records, a Division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "603634",
							"name": "Miley Cyrus",
							"role": "primary_artists",
							"image": "http://c.saavncdn.com/artists/Miley_Cyrus_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/miley-cyrus-songs/FRjYU7Dlr2U_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "603634",
							"name": "Miley Cyrus",
							"role": "music",
							"image": "http://c.saavncdn.com/artists/Miley_Cyrus_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/miley-cyrus-songs/FRjYU7Dlr2U_"
						},
						{
							"id": "1887754",
							"name": "Michael Pollack",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/michael-pollack-songs/kdY6-WgNJ5M_"
						},
						{
							"id": "7770383",
							"name": "Gregory Aldae Hein",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gregory-aldae-hein-songs/oOnOvoD7ia8_"
						},
						{
							"id": "603634",
							"name": "Miley Cyrus",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Miley_Cyrus_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/miley-cyrus-songs/FRjYU7Dlr2U_"
						},
						{
							"id": "603634",
							"name": "Miley Cyrus",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/artists/Miley_Cyrus_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/miley-cyrus-songs/FRjYU7Dlr2U_"
						},
						{
							"id": "1887754",
							"name": "Michael Pollack",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/michael-pollack-songs/kdY6-WgNJ5M_"
						},
						{
							"id": "7770383",
							"name": "Gregory Aldae Hein",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gregory-aldae-hein-songs/oOnOvoD7ia8_"
						}
					]
				},
				"releaseDate": "2023-01-13",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "XOKVoEt6",
				"name": "Never Gonna Not Dance Again",
				"subtitle": "P!nk - TRUSTFALL",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/never-gonna-not-dance-again/KCcgZxt1QwU",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/222/TRUSTFALL-English-2023-20230213235408-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/222/TRUSTFALL-English-2023-20230213235408-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/222/TRUSTFALL-English-2023-20230213235408-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 28939,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Alecia Moore, Shellback, Max Martin",
				"song": null,
				"albumId": "42582928",
				"album": "TRUSTFALL",
				"label": "RCA Records Label",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyiI3uiZMZ1GpCh0FEVweD4W7P47A9UDhaycAJxmNBR5HcQUUlO2qsexw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/trustfall/LTrXB0gFWxs_",
				"duration": 225,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2022 RCA Records, a division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "522531",
							"name": "P!nk",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/P-nk_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/p%21nk-songs/iFMVWBV6PzA_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "522535",
							"name": "Alecia Moore",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/alecia-moore-songs/8Xk3wxDKLO0_"
						},
						{
							"id": "566167",
							"name": "Shellback",
							"role": "music",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "music",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						},
						{
							"id": "522531",
							"name": "P!nk",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/P-nk_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/p%21nk-songs/iFMVWBV6PzA_"
						},
						{
							"id": "522535",
							"name": "Alecia Moore",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/alecia-moore-songs/8Xk3wxDKLO0_"
						},
						{
							"id": "566167",
							"name": "Shellback",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						}
					]
				},
				"releaseDate": "2023-02-17",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "7DFLXzjN",
				"name": "I Ain&#039;t Worried",
				"subtitle": "OneRepublic - I Ain’t Worried (Music From The Motion Picture &quot;Top Gun: Maverick&quot;)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/i-aint-worried/RywtfSxKXX0",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/299/I-Ain-t-Worried-Music-From-The-Motion-Picture-Top-Gun-Maverick-English-2022-20220513084338-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/299/I-Ain-t-Worried-Music-From-The-Motion-Picture-Top-Gun-Maverick-English-2022-20220513084338-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/299/I-Ain-t-Worried-Music-From-The-Motion-Picture-Top-Gun-Maverick-English-2022-20220513084338-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2022,
				"playCount": 732308,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Ryan Tedder, Brent Kutzle, Tyler Spry, John Eriksson, Peter Morén, Björn Yttling",
				"song": null,
				"albumId": "34932276",
				"album": "I Ain’t Worried (Music From The Motion Picture &quot;Top Gun: Maverick&quot;)",
				"label": "Interscope Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy7GBwst82YKGP/RP/tD3RTnJSyMduIp/ILPrmPAZ09tW2F2sa1JL36Bw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/i-ain%e2%80%99t-worried-music-from-the-motion-picture-top-gun%3a-maverick/luz4Qnn2HEM_",
				"duration": 148,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2022 Mosley Music/Interscope Records",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "598351",
							"name": "OneRepublic",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Onerepublic_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/onerepublic-songs/rXNvoiX7x7w_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "565823",
							"name": "Ryan Tedder",
							"role": "music",
							"image": "https://c.saavncdn.com/165/Delicate-Sawyr-And-Ryan-Tedder-Mix--English-2018-20180524160725-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ryan-tedder-songs/0dGUy4bUZIY_"
						},
						{
							"id": "575176",
							"name": "Brent Kutzle",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brent-kutzle-songs/qSyLBwptjok_"
						},
						{
							"id": "6949181",
							"name": "Tyler Spry",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-spry-songs/0A434T5ip,o_"
						},
						{
							"id": "580209",
							"name": "John Eriksson",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/john-eriksson-songs/hOtcVgu5oiM_"
						},
						{
							"id": "580226",
							"name": "Peter Morén",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/peter-moren-songs/6U3r5OaNvjw_"
						},
						{
							"id": "580207",
							"name": "Björn Yttling",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bjorn-yttling-songs/eVkKhYjevic_"
						},
						{
							"id": "598351",
							"name": "OneRepublic",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Onerepublic_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/onerepublic-songs/rXNvoiX7x7w_"
						},
						{
							"id": "565823",
							"name": "Ryan Tedder",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/165/Delicate-Sawyr-And-Ryan-Tedder-Mix--English-2018-20180524160725-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ryan-tedder-songs/0dGUy4bUZIY_"
						},
						{
							"id": "575176",
							"name": "Brent Kutzle",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brent-kutzle-songs/qSyLBwptjok_"
						},
						{
							"id": "6949181",
							"name": "Tyler Spry",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-spry-songs/0A434T5ip,o_"
						},
						{
							"id": "580209",
							"name": "John Eriksson",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/john-eriksson-songs/hOtcVgu5oiM_"
						},
						{
							"id": "580226",
							"name": "Peter Morén",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/peter-moren-songs/6U3r5OaNvjw_"
						},
						{
							"id": "580207",
							"name": "Björn Yttling",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bjorn-yttling-songs/eVkKhYjevic_"
						}
					]
				},
				"releaseDate": "2022-05-13",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "pKtYXO9M",
				"name": "Cake By The Ocean",
				"subtitle": "DNCE - Cake By The Ocean",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/cake-by-the-ocean/ACMfaCx-Dn4",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/434/Cake-By-The-Ocean-English-2015-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/434/Cake-By-The-Ocean-English-2015-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/434/Cake-By-The-Ocean-English-2015-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2015,
				"playCount": 2403218,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "1489744",
				"album": "Cake By The Ocean",
				"label": "Universal Music India Pvt Ltd.",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDydJP3ix1fxlvwKPvwDgz5dOGjhfqZVgjrIYiKlzzLZcfQKRBpUh2Qtxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/cake-by-the-ocean/of-tGPvd76o_",
				"duration": 218,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2015 Republic Records, a division of UMG Recordings, Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "1237366",
							"name": "DNCE",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/DNCE_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dnce-songs/2hs-vhG4ijE_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "1237366",
							"name": "DNCE",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/DNCE_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dnce-songs/2hs-vhG4ijE_"
						}
					]
				},
				"releaseDate": "2015-09-18",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "2I6kShRT",
				"name": "My Stupid Heart",
				"subtitle": "Walk Off The Earth - My Stupid Heart",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/my-stupid-heart/QiFdWidYZWc",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/601/My-Stupid-Heart-English-2023-20230120161447-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/601/My-Stupid-Heart-English-2023-20230120161447-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/601/My-Stupid-Heart-English-2023-20230120161447-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 172596,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Gianni Luminati Nicassio, Sarah Blackwood, Tokyo Speirs, Jake Torrey, Michael Matosic, Lostboy",
				"song": null,
				"albumId": "40704943",
				"album": "My Stupid Heart",
				"label": "Golden Carrot Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDygKaeXrG49g74DgNbKuq5AvrFXfVzjd04MqfUlGbeIzpOzNKz/vj7Mhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/my-stupid-heart/UEoAi3VOM,E_",
				"duration": 160,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2023 Golden Carrot Records",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "578519",
							"name": "Walk Off The Earth",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/240/Old-Town-Road-English-2019-20190417153111-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/walk-off-the-earth-songs/33HnBzPjQyI_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "774073",
							"name": "Gianni Luminati Nicassio",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gianni-luminati-nicassio-songs/cEz2oVWnoYI_"
						},
						{
							"id": "580792",
							"name": "Sarah Blackwood",
							"role": "music",
							"image": "https://c.saavncdn.com/967/Scars-to-Your-Beautiful-English-2016-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sarah-blackwood-songs/Pemsx7AguVU_"
						},
						{
							"id": "7012205",
							"name": "Tokyo Speirs",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tokyo-speirs-songs/YKEZu6cjGnQ_"
						},
						{
							"id": "2721559",
							"name": "Jake Torrey",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jake-torrey-songs/HUTngr,8sN0_"
						},
						{
							"id": "3976592",
							"name": "Michael Matosic",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/michael-matosic-songs/M1PMG1b5SAY_"
						},
						{
							"id": "3337530",
							"name": "Lostboy",
							"role": "music",
							"image": "https://c.saavncdn.com/159/Elysium-English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/lostboy-songs/cazRvLWFBR4_"
						},
						{
							"id": "578519",
							"name": "Walk Off The Earth",
							"role": "singer",
							"image": "https://c.saavncdn.com/240/Old-Town-Road-English-2019-20190417153111-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/walk-off-the-earth-songs/33HnBzPjQyI_"
						}
					]
				},
				"releaseDate": "2023-01-27",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "iixWhNyf",
				"name": "YOU &amp; I (feat. Khalid)",
				"subtitle": "Anne-Marie ft. Khalid - UNHEALTHY (Deluxe)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/you-i-feat.-khalid/GQETZhx,TlU",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/228/UNHEALTHY-Deluxe-English-2023-20230728165039-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/228/UNHEALTHY-Deluxe-English-2023-20230728165039-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/228/UNHEALTHY-Deluxe-English-2023-20230728165039-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 3044,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Anne-Marie Nicholson, Christopher Paul Smith, James Terence Murray, Kamille, Khalid Robinson, Mustafa Armando Ibrahim Omer",
				"song": null,
				"albumId": "46978712",
				"album": "UNHEALTHY (Deluxe)",
				"label": "Atlantic Records UK",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyip0E+Q6ZZTtxy42DeanbQTnZcUUaM41J9yxPnlSTZ4HY+haSn9AbBhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/unhealthy-deluxe/x2krzZpiktg_",
				"duration": 205,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "A Major Toms / Asylum Records UK release, ℗ 2023  Warner Music UK Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "1271180",
							"name": "Anne-Marie",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Anne-Marie_20190923095331_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/anne-marie-songs/J-YCQJt2QXg_"
						}
					],
					"featuredArtists": [
						{
							"id": "464729",
							"name": "Khalid",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/Khalid_20190823071706_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/khalid-songs/EAaDu4WxQfQ_"
						}
					],
					"artists": [
						{
							"id": "765539",
							"name": "Anne-Marie Nicholson",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/anne-marie-nicholson-songs/pJ3JWJGeLLw_"
						},
						{
							"id": "4545516",
							"name": "Christopher Paul Smith",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/christopher-paul-smith-songs/SWSI3VdblI4_"
						},
						{
							"id": "1122778",
							"name": "James Terence Murray",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/james-terence-murray-songs/sAJsNZORXWA_"
						},
						{
							"id": "1223008",
							"name": "Kamille",
							"role": "music",
							"image": "https://c.saavncdn.com/150/Go-Deep-English-2018-20180509230350-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/kamille-songs/uqM5yDRmAgM_"
						},
						{
							"id": "2127113",
							"name": "Khalid Robinson",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Khalid_Robinson_20190927140526_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/khalid-robinson-songs/yIlpHzRxZiU_"
						},
						{
							"id": "1122779",
							"name": "Mustafa Armando Ibrahim Omer",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mustafa-armando-ibrahim-omer-songs/WyLTByeMAJ4_"
						},
						{
							"id": "1271180",
							"name": "Anne-Marie",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Anne-Marie_20190923095331_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/anne-marie-songs/J-YCQJt2QXg_"
						}
					]
				},
				"releaseDate": "2023-07-28",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "qa60YFzN",
				"name": "Made You Look",
				"subtitle": "Meghan Trainor - Takin&#039; It Back",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/made-you-look/AQldAS12TX0",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/188/Takin-It-Back-English-2022-20221015042009-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/188/Takin-It-Back-English-2022-20221015042009-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/188/Takin-It-Back-English-2022-20221015042009-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2022,
				"playCount": 424133,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Meghan Trainor, Federico Vindver, Sean Douglas",
				"song": null,
				"albumId": "39089268",
				"album": "Takin&#039; It Back",
				"label": "Epic",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy4BlqUmwWmm2AKBmpAHYshMFEYduTm6WJsMdBT+jnk5iyAxxPSQfiPxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/takin-it-back/50MTWkjNYdQ_",
				"duration": 134,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2022 Epic Records, a division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "4389613",
							"name": "Federico Vindver",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/federico-vindver-songs/2zUHVMKZxOU_"
						},
						{
							"id": "577478",
							"name": "Sean Douglas",
							"role": "music",
							"image": "https://c.saavncdn.com/683/Ur-Luv-Getz-Me-Hi--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sean-douglas-songs/uHTTQOT4-sQ_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "696615",
							"name": "Meghan Trainor",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Meghan_Trainor_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/meghan-trainor-songs/DQMWktfbHOw_"
						},
						{
							"id": "4389613",
							"name": "Federico Vindver",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/federico-vindver-songs/2zUHVMKZxOU_"
						},
						{
							"id": "577478",
							"name": "Sean Douglas",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/683/Ur-Luv-Getz-Me-Hi--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sean-douglas-songs/uHTTQOT4-sQ_"
						}
					]
				},
				"releaseDate": "2022-10-21",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "wLxoOff5",
				"name": "Uptown Funk",
				"subtitle": "Mark Ronson ft. Bruno Mars - Uptown Funk",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/uptown-funk/ByQTXjtWUQY",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/049/Uptown-Funk-English-2014-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/049/Uptown-Funk-English-2014-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/049/Uptown-Funk-English-2014-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2014,
				"playCount": 7588412,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Mark Ronson, Lonnie Simmons, Ronnie Wilson, Charles Wilson, Jeff Bhasker, Rudolph Taylor, Robert Wilson, Nicholaus Williams, Devon Gallaspy, Philip Lawrence, Peter Hernandez",
				"song": null,
				"albumId": "1205588",
				"album": "Uptown Funk",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyZ8tzxoCTaV06ajomuOq/2U6oL3GUOPeB4xM6isTpYDar1qryIG4rZBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/uptown-funk/zqScnAt-WfY_",
				"duration": 270,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2014 Mark Ronson under exclusive licence to Sony Music Entertainment UK Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "566374",
							"name": "Mark Ronson",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Mark_Ronson_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mark-ronson-songs/MP3neLrI07M_"
						}
					],
					"featuredArtists": [
						{
							"id": "566404",
							"name": "Bruno Mars",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/Bruno_Mars_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bruno-mars-songs/OP9Vnjfx1mM_"
						}
					],
					"artists": [
						{
							"id": "566374",
							"name": "Mark Ronson",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Mark_Ronson_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mark-ronson-songs/MP3neLrI07M_"
						},
						{
							"id": "564834",
							"name": "Lonnie Simmons",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/lonnie-simmons-songs/21Q-hUzIJcQ_"
						},
						{
							"id": "554205",
							"name": "Ronnie Wilson",
							"role": "music",
							"image": "https://c.saavncdn.com/801/They-Didn-t-Know--English-2011-20190412233551-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ronnie-wilson-songs/dFv1Vc-bpn4_"
						},
						{
							"id": "554203",
							"name": "Charles Wilson",
							"role": "music",
							"image": "https://c.saavncdn.com/940/Make-It-Do-Something-English-2011-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/charles-wilson-songs/eReP4cWfttc_"
						},
						{
							"id": "566172",
							"name": "Jeff Bhasker",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jeff-bhasker-songs/lnl3ESr3WQo_"
						},
						{
							"id": "848661",
							"name": "Rudolph Taylor",
							"role": "music",
							"image": "https://c.saavncdn.com/756/I-m-Moving-Out-Fast-Single-English-2012-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/rudolph-taylor-songs/0izTVJ2-xtc_"
						},
						{
							"id": "566272",
							"name": "Robert Wilson",
							"role": "music",
							"image": "https://c.saavncdn.com/393/Angels-Without-Voices-English-2018-20180525200124-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/robert-wilson-songs/g-YFbGAecJs_"
						},
						{
							"id": "782293",
							"name": "Nicholaus Williams",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/nicholaus-williams-songs/qO-6LoyBi74_"
						},
						{
							"id": "782292",
							"name": "Devon Gallaspy",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/devon-gallaspy-songs/I,RQXB-5xE0_"
						},
						{
							"id": "528211",
							"name": "Philip Lawrence",
							"role": "music",
							"image": "https://c.saavncdn.com/837/Letters-I-Never-Sent-English-2013-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/philip-lawrence-songs/MlKieArW3IA_"
						},
						{
							"id": "599159",
							"name": "Peter Hernandez",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/peter-hernandez-songs/4keSNxY2twk_"
						},
						{
							"id": "566374",
							"name": "Mark Ronson",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Mark_Ronson_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mark-ronson-songs/MP3neLrI07M_"
						},
						{
							"id": "566404",
							"name": "Bruno Mars",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Bruno_Mars_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bruno-mars-songs/OP9Vnjfx1mM_"
						},
						{
							"id": "566172",
							"name": "Jeff Bhasker",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jeff-bhasker-songs/lnl3ESr3WQo_"
						},
						{
							"id": "566374",
							"name": "Mark Ronson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Mark_Ronson_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mark-ronson-songs/MP3neLrI07M_"
						},
						{
							"id": "564834",
							"name": "Lonnie Simmons",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/lonnie-simmons-songs/21Q-hUzIJcQ_"
						},
						{
							"id": "554205",
							"name": "Ronnie Wilson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/801/They-Didn-t-Know--English-2011-20190412233551-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ronnie-wilson-songs/dFv1Vc-bpn4_"
						},
						{
							"id": "554203",
							"name": "Charles Wilson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/940/Make-It-Do-Something-English-2011-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/charles-wilson-songs/eReP4cWfttc_"
						},
						{
							"id": "566172",
							"name": "Jeff Bhasker",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jeff-bhasker-songs/lnl3ESr3WQo_"
						},
						{
							"id": "848661",
							"name": "Rudolph Taylor",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/756/I-m-Moving-Out-Fast-Single-English-2012-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/rudolph-taylor-songs/0izTVJ2-xtc_"
						},
						{
							"id": "566272",
							"name": "Robert Wilson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/393/Angels-Without-Voices-English-2018-20180525200124-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/robert-wilson-songs/g-YFbGAecJs_"
						},
						{
							"id": "782293",
							"name": "Nicholaus Williams",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/nicholaus-williams-songs/qO-6LoyBi74_"
						},
						{
							"id": "782292",
							"name": "Devon Gallaspy",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/devon-gallaspy-songs/I,RQXB-5xE0_"
						},
						{
							"id": "528211",
							"name": "Philip Lawrence",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/837/Letters-I-Never-Sent-English-2013-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/philip-lawrence-songs/MlKieArW3IA_"
						},
						{
							"id": "599159",
							"name": "Peter Hernandez",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/peter-hernandez-songs/4keSNxY2twk_"
						}
					]
				},
				"releaseDate": "2014-11-10",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "cXG8RHdZ",
				"name": "Seven (feat. Latto) (Clean Ver.)",
				"subtitle": "Jung Kook, Latto - Seven (feat. Latto)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/seven-feat.-latto-clean-ver./EzAsCSZ4U2k",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 307007,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Andrew Watt, Jon Bellion, Henry Walter, Theron Makiel Thomas, Latto",
				"song": null,
				"albumId": "46690567",
				"album": "Seven (feat. Latto)",
				"label": "BIGHIT MUSIC",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyd3neTX1RK7Y84b8htoRSPnp9ixR5wzKDQaMQYC3RqPDUJUt92I6Yyxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/seven-feat.-latto/J3WMP6qyz4o_",
				"duration": 184,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "© 2023 BIGHIT MUSIC",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "8711782",
							"name": "Jung Kook",
							"role": "primary_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jung-kook-songs/H-uZ66aVWjo_"
						},
						{
							"id": "1437962",
							"name": "Latto",
							"role": "primary_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/latto-songs/1JYqF,NlmlU_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "777522",
							"name": "Andrew Watt",
							"role": "music",
							"image": "https://c.saavncdn.com/848/Wild-Youth-English-2016-20170826004007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/andrew-watt-songs/PxEA4NI97bU_"
						},
						{
							"id": "843751",
							"name": "Jon Bellion",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Jon_Bellion_20200218145312_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jon-bellion-songs/JNMR2pR9reQ_"
						},
						{
							"id": "573597",
							"name": "Henry Walter",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/henry-walter-songs/nLRg5wWo2lI_"
						},
						{
							"id": "982938",
							"name": "Theron Makiel Thomas",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/theron-makiel-thomas-songs/pZuNO7-9wn0_"
						},
						{
							"id": "1437962",
							"name": "Latto",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/latto-songs/1JYqF,NlmlU_"
						},
						{
							"id": "8711782",
							"name": "Jung Kook",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jung-kook-songs/H-uZ66aVWjo_"
						},
						{
							"id": "1437962",
							"name": "Latto",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/latto-songs/1JYqF,NlmlU_"
						}
					]
				},
				"releaseDate": "2023-07-14",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "rMKwZimO",
				"name": "Happy (From &quot;Despicable Me 2&quot;)",
				"subtitle": "Pharrell Williams - G I R L",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/happy-from-despicable-me-2/AiUgRi5ZWnw",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/877/G-I-R-L-English-2014-20190624155133-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/877/G-I-R-L-English-2014-20190624155133-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/877/G-I-R-L-English-2014-20190624155133-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2014,
				"playCount": 1935391,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Pharrell Williams",
				"song": null,
				"albumId": "16288577",
				"album": "G I R L",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyHon94I6zjfsANlSaQSS6S4X8vGpZcK9nFjsTVoHIAznj5kf0JXdmxRw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/g-i-r-l/0ixQvvSvNJQ_",
				"duration": 233,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2013 Back Lot Music, under exclusive license to Columbia Records, a Division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						}
					]
				},
				"releaseDate": "2014-03-03",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "IOb4m8dA",
				"name": "Dance The Night (From Barbie The Album)",
				"subtitle": "Dua Lipa - Dance The Night (From Barbie The Album)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/dance-the-night-from-barbie-the-album/OScJBRkIU3I",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 125951,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "45537304",
				"album": "Dance The Night (From Barbie The Album)",
				"label": "Atlantic Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyrDThlGeSlnon+ODdm1PQHnS83Ry/mCK4npCXlqOSf8hJeiein2CeMxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/dance-the-night-from-barbie-the-album/,d3fJIbyrjU_",
				"duration": 176,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2023 Atlantic Recording Corporation, Warner Bros. Entertainment, Inc. & Mattel, Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "1274170",
							"name": "Dua Lipa",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Dua_Lipa_003_20200226102459_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dua-lipa-songs/r-OWIKgpX2I_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "1274170",
							"name": "Dua Lipa",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Dua_Lipa_003_20200226102459_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dua-lipa-songs/r-OWIKgpX2I_"
						}
					]
				},
				"releaseDate": "2023-05-25",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "N34NxTO9",
				"name": "Watermelon Sugar",
				"subtitle": "Harry Styles - Fine Line",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/watermelon-sugar/PltffwxkeAo",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/213/Fine-Line-English-2019-20191211223631-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/213/Fine-Line-English-2019-20191211223631-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/213/Fine-Line-English-2019-20191211223631-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2019,
				"playCount": 2630506,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Harry Styles, Mitch Rowland, Thomas Hull, Tyler Johnson",
				"song": null,
				"albumId": "18288549",
				"album": "Fine Line",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyeCytIyGLY9/r28xJhLok7FjHzeRumt2ryJkQxhjIuIXKAILD6X4yyxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/fine-line/y9AFL,ralvA_",
				"duration": 173,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "I don't know if I could ever go without",
				"starred": false,
				"copyrightText": "(P) 2019 Erskine Records Limited, under exclusive license to Columbia Records, a Division of Sony Music Entertainment",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "3163449",
							"name": "Mitch Rowland",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mitch-rowland-songs/whyIrmTwfOw_"
						},
						{
							"id": "866653",
							"name": "Thomas Hull",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/thomas-hull-songs/f68P6Jg4CnY_"
						},
						{
							"id": "672872",
							"name": "Tyler Johnson",
							"role": "music",
							"image": "https://c.saavncdn.com/252/Grenade-Acoustic-Guitar--English-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-johnson-songs/std82FJ6ROw_"
						},
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "577199",
							"name": "Harry Styles",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Harry_Styles_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harry-styles-songs/vVtHXctbA,o_"
						},
						{
							"id": "3163449",
							"name": "Mitch Rowland",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mitch-rowland-songs/whyIrmTwfOw_"
						},
						{
							"id": "866653",
							"name": "Thomas Hull",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/thomas-hull-songs/f68P6Jg4CnY_"
						},
						{
							"id": "672872",
							"name": "Tyler Johnson",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/252/Grenade-Acoustic-Guitar--English-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tyler-johnson-songs/std82FJ6ROw_"
						}
					]
				},
				"releaseDate": "2019-12-13",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "Ikxrjgm-",
				"name": "Don&#039;t You Need Somebody (feat. Enrique Iglesias, R. City, Serayah &amp; Shaggy)",
				"subtitle": "Redone ft. R. City, Serayah, Enrique Iglesias, Shaggy - Don&#039;t You Need Somebody (feat. Enrique Iglesias, R. City, Serayah &amp; Shaggy)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/dont-you-need-somebody-feat.-enrique-iglesias-r.-city-serayah-shaggy/OQMTQx5XWh4",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/755/Don-t-You-Need-Somebody-feat-Enrique-Iglesias-R-City-Serayah-Shaggy-English-2016-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/755/Don-t-You-Need-Somebody-feat-Enrique-Iglesias-R-City-Serayah-Shaggy-English-2016-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/755/Don-t-You-Need-Somebody-feat-Enrique-Iglesias-R-City-Serayah-Shaggy-English-2016-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2016,
				"playCount": 16243470,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "2338456",
				"album": "Don&#039;t You Need Somebody (feat. Enrique Iglesias, R. City, Serayah &amp; Shaggy)",
				"label": "Warner Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyxiS5y+mqPHdqHPFramJDSwlyrOA+7inbv8IksqGhMvuruRBvkARknBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/dont-you-need-somebody-feat.-enrique-iglesias-r.-city-serayah-shaggy/ltE5fN6h7iI_",
				"duration": 207,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2016 Warner Records Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "564811",
							"name": "Redone",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Redone_000_20191129083743_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/redone-songs/-3BSqWEHt84_"
						}
					],
					"featuredArtists": [
						{
							"id": "753769",
							"name": "R. City",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/R_City_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/r.-city-songs/Gd3VgdJf5-4_"
						},
						{
							"id": "1171860",
							"name": "Serayah",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/Serayah_000_20191129100149_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/serayah-songs/ZUW3iRB8VFg_"
						},
						{
							"id": "565534",
							"name": "Enrique Iglesias",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/Enrique_Iglesias_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/enrique-iglesias-songs/4IFUBQ46K8c_"
						},
						{
							"id": "589039",
							"name": "Shaggy",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/Shaggy_000_20191129101158_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shaggy-songs/CvoCyjr3Lvg_"
						}
					],
					"artists": [
						{
							"id": "564811",
							"name": "Redone",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Redone_000_20191129083743_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/redone-songs/-3BSqWEHt84_"
						},
						{
							"id": "753769",
							"name": "R. City",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/R_City_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/r.-city-songs/Gd3VgdJf5-4_"
						},
						{
							"id": "1171860",
							"name": "Serayah",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Serayah_000_20191129100149_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/serayah-songs/ZUW3iRB8VFg_"
						},
						{
							"id": "565534",
							"name": "Enrique Iglesias",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Enrique_Iglesias_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/enrique-iglesias-songs/4IFUBQ46K8c_"
						},
						{
							"id": "589039",
							"name": "Shaggy",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Shaggy_000_20191129101158_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shaggy-songs/CvoCyjr3Lvg_"
						}
					]
				},
				"releaseDate": "2016-05-20",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "_3pNmLZN",
				"name": "CAN&#039;T STOP THE FEELING! (from DreamWorks Animation&#039;s &quot;TROLLS&quot;)",
				"subtitle": "Justin Timberlake - CAN&#039;T STOP THE FEELING! (from DreamWorks Animation&#039;s &quot;TROLLS&quot;)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/cant-stop-the-feeling%21-from-dreamworks-animations-trolls/L1sbfxl8bX0",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/653/Can-t-Stop-the-Feeling-Original-Song-from-DreamWorks-Animation-TROLLS-English-2016-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/653/Can-t-Stop-the-Feeling-Original-Song-from-DreamWorks-Animation-TROLLS-English-2016-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/653/Can-t-Stop-the-Feeling-Original-Song-from-DreamWorks-Animation-TROLLS-English-2016-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2016,
				"playCount": 4861582,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Justin Timberlake, Shellback, Max Martin",
				"song": null,
				"albumId": "2293960",
				"album": "CAN&#039;T STOP THE FEELING! (from DreamWorks Animation&#039;s &quot;TROLLS&quot;)",
				"label": "Villa 40/RCA Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyFbjpFqoGn2YhYJ/m96UAQZW4jH1CeE5kTv84ZtW2F76c4TQydD2RrBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/cant-stop-the-feeling%21-from-dreamworks-animations-trolls/Ee2mHE8dZVE_",
				"duration": 236,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "So just dance, dance, dance, come on",
				"starred": false,
				"copyrightText": "(P) 2016 RCA Records/DreamWorks Animation LLC",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "522130",
							"name": "Justin Timberlake",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Justin_Timberlake_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-timberlake-songs/WA1w,uEuQFA_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "522130",
							"name": "Justin Timberlake",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Justin_Timberlake_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-timberlake-songs/WA1w,uEuQFA_"
						},
						{
							"id": "566167",
							"name": "Shellback",
							"role": "music",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "music",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						},
						{
							"id": "522130",
							"name": "Justin Timberlake",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Justin_Timberlake_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-timberlake-songs/WA1w,uEuQFA_"
						},
						{
							"id": "837668",
							"name": "Wojtek Goral",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/wojtek-goral-songs/Ai4iXYOsmCA_"
						},
						{
							"id": "1814094",
							"name": "Mattias Bylund",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mattias-bylund-songs/XnSLQNgo9Zo_"
						},
						{
							"id": "522130",
							"name": "Justin Timberlake",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Justin_Timberlake_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-timberlake-songs/WA1w,uEuQFA_"
						},
						{
							"id": "566167",
							"name": "Shellback",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						}
					]
				},
				"releaseDate": "2016-05-06",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "HnUx9z42",
				"name": "About Damn Time",
				"subtitle": "Lizzo - Special",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/about-damn-time/OAY,SU1KAwE",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2022,
				"playCount": 332807,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "36493120",
				"album": "Special",
				"label": "Nice Life/Atlantic",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyp7IajhqwFdJh98usIcF4kZ6JLrpy+0+2J1DIR4kHHoCvUyFGoguqrhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/special/eZ1mCKwjnLQ_",
				"duration": 191,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2021 Nice Life Recording Company and Atlantic Recording Corporation",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "727805",
							"name": "Lizzo",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Lizzo_20200217102408_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/lizzo-songs/SgirpeHhPvo_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "727805",
							"name": "Lizzo",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Lizzo_20200217102408_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/lizzo-songs/SgirpeHhPvo_"
						}
					]
				},
				"releaseDate": "2022-07-15",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "o1qyko3N",
				"name": "Peaches",
				"subtitle": "Justin Bieber ft. Daniel Caesar, Giveon - Justice",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/peaches/H1kaSB9fBH0",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2021,
				"playCount": 21591634,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Justin Bieber, Andrew Wotman, Ashton Simmonds, Giveon Evans, Bernard Harvey, Luis Manuel Martinez Jr, Louis Bell, Felisha King, Matthew Sean Leon, Keavan Yazdani, Aaron Simmonds",
				"song": null,
				"albumId": "26030221",
				"album": "Justice",
				"label": "RBMG/Def Jam",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDysUHOLcVmXRn/JwFmwF9YlthPiT6XIUbDpNpqPenX6iNTom9t6TUdZRw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/justice/7R,IUpSYb,0_",
				"duration": 198,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2021 Def Jam Recordings, a division of UMG Recordings, Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "568565",
							"name": "Justin Bieber",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Justin_Bieber_005_20201127112218_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-bieber-songs/deJJWFd1ItE_"
						}
					],
					"featuredArtists": [
						{
							"id": "777121",
							"name": "Daniel Caesar",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/305/Get-You-Single-English-2016-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/daniel-caesar-songs/gbgKh4vMHvk_"
						},
						{
							"id": "5181290",
							"name": "Giveon",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/965/Garden-Kisses-English-2018-20180720045232-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/giveon-songs/R4jQDKdox08_"
						}
					],
					"artists": [
						{
							"id": "568565",
							"name": "Justin Bieber",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Justin_Bieber_005_20201127112218_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-bieber-songs/deJJWFd1ItE_"
						},
						{
							"id": "1256966",
							"name": "Andrew Wotman",
							"role": "music",
							"image": "https://c.saavncdn.com/714/On-My-Own-English-2008-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/andrew-wotman-songs/IKutWGGxYdI_"
						},
						{
							"id": "3254584",
							"name": "Ashton Simmonds",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ashton-simmonds-songs/bbP6Ua-ZUGQ_"
						},
						{
							"id": "7242411",
							"name": "Giveon Evans",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/giveon-evans-songs/ChwnbSsDm4s_"
						},
						{
							"id": "607716",
							"name": "Bernard Harvey",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bernard-harvey-songs/3kJPna3K75M_"
						},
						{
							"id": "5231029",
							"name": "Luis Manuel Martinez Jr",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/luis-manuel-martinez-jr-songs/ItbwpNU1Hts_"
						},
						{
							"id": "1256967",
							"name": "Louis Bell",
							"role": "music",
							"image": "https://c.saavncdn.com/997/All-Power-EP-English-2016-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louis-bell-songs/1CuDia,L968_"
						},
						{
							"id": "566278",
							"name": "Felisha King",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/felisha-king-songs/581q8Emg5Cc_"
						},
						{
							"id": "7565411",
							"name": "Matthew Sean Leon",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/matthew-sean-leon-songs/KeZ-MNMJ9V0_"
						},
						{
							"id": "10150976",
							"name": "Keavan Yazdani",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/keavan-yazdani-songs/XhRGwXJqYgM_"
						},
						{
							"id": "10177553",
							"name": "Aaron Simmonds",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/aaron-simmonds-songs/0NURMujGOr4_"
						},
						{
							"id": "568565",
							"name": "Justin Bieber",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Justin_Bieber_005_20201127112218_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-bieber-songs/deJJWFd1ItE_"
						},
						{
							"id": "568565",
							"name": "Justin Bieber",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Justin_Bieber_005_20201127112218_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/justin-bieber-songs/deJJWFd1ItE_"
						},
						{
							"id": "1256966",
							"name": "Andrew Wotman",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/714/On-My-Own-English-2008-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/andrew-wotman-songs/IKutWGGxYdI_"
						},
						{
							"id": "3254584",
							"name": "Ashton Simmonds",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ashton-simmonds-songs/bbP6Ua-ZUGQ_"
						},
						{
							"id": "7242411",
							"name": "Giveon Evans",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/giveon-evans-songs/ChwnbSsDm4s_"
						},
						{
							"id": "607716",
							"name": "Bernard Harvey",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/bernard-harvey-songs/3kJPna3K75M_"
						},
						{
							"id": "5231029",
							"name": "Luis Manuel Martinez Jr",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/luis-manuel-martinez-jr-songs/ItbwpNU1Hts_"
						},
						{
							"id": "1256967",
							"name": "Louis Bell",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/997/All-Power-EP-English-2016-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louis-bell-songs/1CuDia,L968_"
						},
						{
							"id": "566278",
							"name": "Felisha King",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/felisha-king-songs/581q8Emg5Cc_"
						},
						{
							"id": "7565411",
							"name": "Matthew Sean Leon",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/matthew-sean-leon-songs/KeZ-MNMJ9V0_"
						},
						{
							"id": "10150976",
							"name": "Keavan Yazdani",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/keavan-yazdani-songs/XhRGwXJqYgM_"
						},
						{
							"id": "10177553",
							"name": "Aaron Simmonds",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/aaron-simmonds-songs/0NURMujGOr4_"
						}
					]
				},
				"releaseDate": "2021-03-19",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "vnPJtKBN",
				"name": "Beautiful People (feat. Khalid)",
				"subtitle": "Ed Sheeran ft. Khalid - Beautiful People (feat. Khalid)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/beautiful-people-feat.-khalid/BgY7ewB7dX0",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/843/Beautiful-People-feat-Khalid--English-2019-20190627093228-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/843/Beautiful-People-feat-Khalid--English-2019-20190627093228-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/843/Beautiful-People-feat-Khalid--English-2019-20190627093228-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2019,
				"playCount": 10974922,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "16391092",
				"album": "Beautiful People (feat. Khalid)",
				"label": "Atlantic Records UK",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyEgQdbJRWOuPcYArSAEtwQrYAUBYvRijDD2WPXHQzl8k4d2vppF3EThw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/beautiful-people-feat.-khalid/t0bM,HF8iXo_",
				"duration": 197,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "2019 An Asylum Records UK release, a division of Atlantic Records UK; ℗ 2019 Warner Music UK Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "578407",
							"name": "Ed Sheeran",
							"role": "primary_artists",
							"image": "http://c.saavncdn.com/artists/Ed_Sheeran_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ed-sheeran-songs/bWIDsVrU6DE_"
						}
					],
					"featuredArtists": [
						{
							"id": "464729",
							"name": "Khalid",
							"role": "featured_artists",
							"image": "http://c.saavncdn.com/artists/Khalid_20190823071706_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/khalid-songs/EAaDu4WxQfQ_"
						}
					],
					"artists": [
						{
							"id": "578407",
							"name": "Ed Sheeran",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Ed_Sheeran_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ed-sheeran-songs/bWIDsVrU6DE_"
						},
						{
							"id": "464729",
							"name": "Khalid",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Khalid_20190823071706_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/khalid-songs/EAaDu4WxQfQ_"
						}
					]
				},
				"releaseDate": "2019-06-28",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "CwWRsyYR",
				"name": "Chemical",
				"subtitle": "Post Malone - Chemical",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/chemical/Mx88YwdJbmE",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/053/Chemical-English-2023-20230414064008-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/053/Chemical-English-2023-20230414064008-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/053/Chemical-English-2023-20230414064008-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 117626,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Louis Bell, Andrew Watt, Austin Post, Billy Walsh",
				"song": null,
				"albumId": "44474214",
				"album": "Chemical",
				"label": "Mercury Records/Republic Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDysyQxTXgfw94sPYRTxIjzyqXgE6m0EAm8vRXzBgOIhFoc99UaL4B/7Bw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/chemical/f8r2HPw,jsk_",
				"duration": 183,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "Tell you that I'm sorry, tell me what I gotta do",
				"starred": false,
				"copyrightText": "℗ 2023 Mercury Records/Republic Records, a division of UMG Recordings, Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "910050",
							"name": "Post Malone",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Post_Malone_004_20190911070147_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/post-malone-songs/Mbk23vKPwtY_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "1256967",
							"name": "Louis Bell",
							"role": "music",
							"image": "https://c.saavncdn.com/997/All-Power-EP-English-2016-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louis-bell-songs/1CuDia,L968_"
						},
						{
							"id": "777522",
							"name": "Andrew Watt",
							"role": "music",
							"image": "https://c.saavncdn.com/848/Wild-Youth-English-2016-20170826004007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/andrew-watt-songs/PxEA4NI97bU_"
						},
						{
							"id": "910051",
							"name": "Austin Post",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/austin-post-songs/Pcv8Zj-yCdY_"
						},
						{
							"id": "876346",
							"name": "Billy Walsh",
							"role": "music",
							"image": "https://c.saavncdn.com/758/Irish-English-2008-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/billy-walsh-songs/pmWCqwQBwnw_"
						},
						{
							"id": "910050",
							"name": "Post Malone",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Post_Malone_004_20190911070147_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/post-malone-songs/Mbk23vKPwtY_"
						},
						{
							"id": "1256967",
							"name": "Louis Bell",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/997/All-Power-EP-English-2016-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louis-bell-songs/1CuDia,L968_"
						},
						{
							"id": "777522",
							"name": "Andrew Watt",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/848/Wild-Youth-English-2016-20170826004007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/andrew-watt-songs/PxEA4NI97bU_"
						},
						{
							"id": "910051",
							"name": "Austin Post",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/austin-post-songs/Pcv8Zj-yCdY_"
						},
						{
							"id": "876346",
							"name": "Billy Walsh",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/758/Irish-English-2008-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/billy-walsh-songs/pmWCqwQBwnw_"
						}
					]
				},
				"releaseDate": "2023-04-14",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "aENw6t08",
				"name": "Shake It Off",
				"subtitle": "Taylor Swift - 1989 (Deluxe)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/shake-it-off/ES0lRkJEBws",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/893/1989-Deluxe--English-2014-20190420034558-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/893/1989-Deluxe--English-2014-20190420034558-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/893/1989-Deluxe--English-2014-20190420034558-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2014,
				"playCount": 2650352,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Shellback, Max Martin, Taylor Swift",
				"song": null,
				"albumId": "10970044",
				"album": "1989 (Deluxe)",
				"label": "Big Machine Records, LLC",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyToByeaArpxVXWsjjoFEyn2Gszp7WhjcL/+OX7c94Ei5Vz5twkNBrUBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/1989-deluxe/23yaqMvxxkc_",
				"duration": 219,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "Baby, I'm just gonna shake, shake, shake, shake, shake",
				"starred": false,
				"copyrightText": "℗ 2014 Big Machine Records, LLC.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "565990",
							"name": "Taylor Swift",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Taylor_Swift_003_20200226074119_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/taylor-swift-songs/bQVPhRbZO1I_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "566167",
							"name": "Shellback",
							"role": "music",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "music",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						},
						{
							"id": "565990",
							"name": "Taylor Swift",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Taylor_Swift_003_20200226074119_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/taylor-swift-songs/bQVPhRbZO1I_"
						},
						{
							"id": "565990",
							"name": "Taylor Swift",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Taylor_Swift_003_20200226074119_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/taylor-swift-songs/bQVPhRbZO1I_"
						},
						{
							"id": "566167",
							"name": "Shellback",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/462/From-Disco-to-House-Dance-Grooves-Flavoured-by-Funky-Disco--English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shellback-songs/uKtrAG4iLRY_"
						},
						{
							"id": "511907",
							"name": "Max Martin",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/848/Beautiful-Day-English-2009-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/max-martin-songs/NuQcWCsPYYM_"
						},
						{
							"id": "565990",
							"name": "Taylor Swift",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Taylor_Swift_003_20200226074119_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/taylor-swift-songs/bQVPhRbZO1I_"
						}
					]
				},
				"releaseDate": "2014-10-27",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "UF6tbTyG",
				"name": "Popular (Music from the HBO Original Series)",
				"subtitle": "The Weeknd, Madonna ft. Playboi Carti - Popular (Music from the HBO Original Series)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/popular-music-from-the-hbo-original-series/JS5dRRZkTnQ",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/802/Popular-From-The-Idol-Vol-1-Music-from-the-HBO-Original-Series-English-2023-20230610103407-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/802/Popular-From-The-Idol-Vol-1-Music-from-the-HBO-Original-Series-English-2023-20230610103407-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/802/Popular-From-The-Idol-Vol-1-Music-from-the-HBO-Original-Series-English-2023-20230610103407-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 183192,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Mike Dean, Leland Wayne, Abel Tesfaye, Sam Levinson, Jordan Carter, Johnny Flippin, Michael Walker",
				"song": null,
				"albumId": "45845199",
				"album": "Popular (Music from the HBO Original Series)",
				"label": "XO / Republic Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyM901aaodZR0ojhrIhQ5QTHejnhlQYnm8BCTePu7ZEnmsbBl5q/JxZxw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/popular-music-from-the-hbo-original-series/-Dw9Ev5U2Fs_",
				"duration": 215,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2023 The Weeknd XO, Inc., Marketed by Republic Records, a Division of UMG Recordings, Inc.",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "615155",
							"name": "The Weeknd",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/The_Weeknd_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/the-weeknd-songs/FJRb7GbYWrQ_"
						},
						{
							"id": "522744",
							"name": "Madonna",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Madonna_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/madonna-songs/ygEp5MhUSFw_"
						}
					],
					"featuredArtists": [
						{
							"id": "1659238",
							"name": "Playboi Carti",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/831/Playboi-Carti-English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/playboi-carti-songs/uYoFTjJe9yE_"
						}
					],
					"artists": [
						{
							"id": "526434",
							"name": "Mike Dean",
							"role": "music",
							"image": "https://c.saavncdn.com/233/Closer-to-Eternity-English-2009-20170916133436-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mike-dean-songs/zB6OkDnUtao_"
						},
						{
							"id": "700267",
							"name": "Leland Wayne",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/leland-wayne-songs/ZdNzlDKLquY_"
						},
						{
							"id": "615156",
							"name": "Abel Tesfaye",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/abel-tesfaye-songs/chP5O9M7POQ_"
						},
						{
							"id": "12294273",
							"name": "Sam Levinson",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sam-levinson-songs/L,nrof7JvWg_"
						},
						{
							"id": "1445580",
							"name": "Jordan Carter",
							"role": "music",
							"image": "https://c.saavncdn.com/522/Tats-English-2017-20171006173253-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jordan-carter-songs/cYGQn9JFUIw_"
						},
						{
							"id": "615700",
							"name": "Johnny Flippin",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/johnny-flippin-songs/-3JJIv2RMhk_"
						},
						{
							"id": "566476",
							"name": "Michael Walker",
							"role": "music",
							"image": "https://c.saavncdn.com/629/Jesus-is-The-Lamb-of-God-English-2018-20180425143427-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/michael-walker-songs/dqxtJjKCIUE_"
						},
						{
							"id": "615155",
							"name": "The Weeknd",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/The_Weeknd_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/the-weeknd-songs/FJRb7GbYWrQ_"
						},
						{
							"id": "522744",
							"name": "Madonna",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Madonna_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/madonna-songs/ygEp5MhUSFw_"
						},
						{
							"id": "526434",
							"name": "Mike Dean",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/233/Closer-to-Eternity-English-2009-20170916133436-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mike-dean-songs/zB6OkDnUtao_"
						},
						{
							"id": "700267",
							"name": "Leland Wayne",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/leland-wayne-songs/ZdNzlDKLquY_"
						},
						{
							"id": "615156",
							"name": "Abel Tesfaye",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/abel-tesfaye-songs/chP5O9M7POQ_"
						},
						{
							"id": "12294273",
							"name": "Sam Levinson",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sam-levinson-songs/L,nrof7JvWg_"
						},
						{
							"id": "1445580",
							"name": "Jordan Carter",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/522/Tats-English-2017-20171006173253-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jordan-carter-songs/cYGQn9JFUIw_"
						},
						{
							"id": "615700",
							"name": "Johnny Flippin",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/johnny-flippin-songs/-3JJIv2RMhk_"
						},
						{
							"id": "566476",
							"name": "Michael Walker",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/629/Jesus-is-The-Lamb-of-God-English-2018-20180425143427-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/michael-walker-songs/dqxtJjKCIUE_"
						}
					]
				},
				"releaseDate": "2023-06-10",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "XjoPd0Z8",
				"name": "Cupid",
				"subtitle": "Fifty Fifty - The Beginning: Cupid",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/cupid/KAIEYRAAbQs",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 490503,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Adam Von Mentzer, Louise Udin, Mac Felländer-Tsai",
				"song": null,
				"albumId": "44013290",
				"album": "The Beginning: Cupid",
				"label": "WM Korea",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyuxI6LAgKLopqX7XgoLHQuahsdsfgBrGNmi8PNnNvLVzZA9z63gpAcRw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/the-beginning%3a-cupid/8oz6VEXT0mI_",
				"duration": 174,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": true,
				"lyricsSnippet": "I gave a second chance to Cupid",
				"starred": false,
				"copyrightText": "℗ 2023 ATTRAKT under exclusive license to Warner Music Korea",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "1894910",
							"name": "Fifty Fifty",
							"role": "primary_artists",
							"image": "http://c.saavncdn.com/158/Longing-English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/fifty-fifty-songs/JJbzEGjRaQM_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "11399354",
							"name": "Adam von Mentzer",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-von-mentzer-songs/CEYusj6xuEs_"
						},
						{
							"id": "7842705",
							"name": "Louise Udin",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louise-udin-songs/,i,GxJbPVOw_"
						},
						{
							"id": "6642479",
							"name": "Mac Felländer-Tsai",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mac-fellander-tsai-songs/NRN5xI7Ej0g_"
						},
						{
							"id": "1894910",
							"name": "Fifty Fifty",
							"role": "singer",
							"image": "http://c.saavncdn.com/158/Longing-English-2017-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/fifty-fifty-songs/JJbzEGjRaQM_"
						},
						{
							"id": "4753306",
							"name": "Ahin",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ahin-songs/iGdx4rER8dE_"
						},
						{
							"id": "11399354",
							"name": "Adam von Mentzer",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-von-mentzer-songs/CEYusj6xuEs_"
						},
						{
							"id": "1893925",
							"name": "Keena",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/keena-songs/et6m2UlG0Qk_"
						},
						{
							"id": "7842705",
							"name": "Louise Udin",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/louise-udin-songs/,i,GxJbPVOw_"
						},
						{
							"id": "6642479",
							"name": "Mac Felländer-Tsai",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mac-fellander-tsai-songs/NRN5xI7Ej0g_"
						},
						{
							"id": "9210213",
							"name": "SIAHN",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/siahn-songs/8,NcOAVpMgc_"
						}
					]
				},
				"releaseDate": "2023-02-24",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "idoLJICu",
				"name": "Girls Like You",
				"subtitle": "Maroon 5 ft. Cardi B - Girls Like You",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/girls-like-you/GQwEfT55dEY",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/016/Girls-Like-You-English-2018-20180529030200-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/016/Girls-Like-You-English-2018-20180529030200-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/016/Girls-Like-You-English-2018-20180529030200-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2018,
				"playCount": 48884204,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Jason Evigan, Adam Levine, Henry Walter, Brittany Talia Hazzard, Gian Stone, Belcalis Almanzar",
				"song": null,
				"albumId": "12975468",
				"album": "Girls Like You",
				"label": "Interscope Records*",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDygijD/m89xH0ysiesh5n0SM0nlQq3JbUjygW8nNjsSjSPwo++K9XZqhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/girls-like-you/Z1SllYQkM24_",
				"duration": 236,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2018 Interscope Records (222 Records)",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "597966",
							"name": "Maroon 5",
							"role": "primary_artists",
							"image": "http://c.saavncdn.com/artists/Maroon_5_003_20200822051619_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/maroon-5-songs/3kopIqasuvA_"
						}
					],
					"featuredArtists": [
						{
							"id": "1261541",
							"name": "Cardi B",
							"role": "featured_artists",
							"image": "http://c.saavncdn.com/artists/Cardi_B_001_20200921152509_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/cardi-b-songs/hWQq3QGVd3g_"
						}
					],
					"artists": [
						{
							"id": "591993",
							"name": "Jason Evigan",
							"role": "music",
							"image": "http://c.saavncdn.com/949/Teen-Beach-Movie-English-2013-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jason-evigan-songs/,zVmyXajdpg_"
						},
						{
							"id": "574298",
							"name": "Adam Levine",
							"role": "music",
							"image": "http://c.saavncdn.com/artists/Adam_Levine_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-levine-songs/IIklhyCcIik_"
						},
						{
							"id": "573597",
							"name": "Henry Walter",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/henry-walter-songs/nLRg5wWo2lI_"
						},
						{
							"id": "801122",
							"name": "Brittany Talia Hazzard",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brittany-talia-hazzard-songs/yUqPTizz1iw_"
						},
						{
							"id": "3788329",
							"name": "Gian Stone",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gian-stone-songs/1-6N,jwQeJM_"
						},
						{
							"id": "1978121",
							"name": "Belcalis Almanzar",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/belcalis-almanzar-songs/n9eFzBh0I84_"
						},
						{
							"id": "597966",
							"name": "Maroon 5",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Maroon_5_003_20200822051619_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/maroon-5-songs/3kopIqasuvA_"
						},
						{
							"id": "591993",
							"name": "Jason Evigan",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/949/Teen-Beach-Movie-English-2013-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jason-evigan-songs/,zVmyXajdpg_"
						},
						{
							"id": "574298",
							"name": "Adam Levine",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/artists/Adam_Levine_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-levine-songs/IIklhyCcIik_"
						},
						{
							"id": "573597",
							"name": "Henry Walter",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/henry-walter-songs/nLRg5wWo2lI_"
						},
						{
							"id": "801122",
							"name": "Brittany Talia Hazzard",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brittany-talia-hazzard-songs/yUqPTizz1iw_"
						},
						{
							"id": "3788329",
							"name": "Gian Stone",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gian-stone-songs/1-6N,jwQeJM_"
						},
						{
							"id": "1978121",
							"name": "Belcalis Almanzar",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/belcalis-almanzar-songs/n9eFzBh0I84_"
						}
					]
				},
				"releaseDate": "2018-05-31",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "k4ejAV55",
				"name": "Do It Like That",
				"subtitle": "TOMORROW X TOGETHER, Jonas Brothers - Do It Like That",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/do-it-like-that/G1wOWzVmAgY",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/627/Do-It-Like-That-English-2023-20230707093538-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/627/Do-It-Like-That-English-2023-20230707093538-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/627/Do-It-Like-That-English-2023-20230707093538-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2023,
				"playCount": 27225,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Ryan Tedder, Grant Boutin, COLETON RUBIN",
				"song": null,
				"albumId": "46529909",
				"album": "Do It Like That",
				"label": "Republic Records - TXT",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyt1TIwBz5mVByN2pVdN6168hHOet5kMao+HGTCv/GFa+/F9gY83/6+hw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/do-it-like-that/LqZSmXbGt0M_",
				"duration": 145,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "© 2023 BIGHIT MUSIC",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "6103174",
							"name": "TOMORROW X TOGETHER",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/TOMORROW_X_TOGETHER_001_20230113181943_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tomorrow-x-together-songs/UDH97R60FVo_"
						},
						{
							"id": "603641",
							"name": "Jonas Brothers",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Jonas_Brothers_002_20200616105406_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jonas-brothers-songs/3LJVYH-rtJY_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "565823",
							"name": "Ryan Tedder",
							"role": "music",
							"image": "https://c.saavncdn.com/165/Delicate-Sawyr-And-Ryan-Tedder-Mix--English-2018-20180524160725-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/ryan-tedder-songs/0dGUy4bUZIY_"
						},
						{
							"id": "2990751",
							"name": "Grant Boutin",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/grant-boutin-songs/MpaxPRlprN0_"
						},
						{
							"id": "7992729",
							"name": "COLETON RUBIN",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/coleton-rubin-songs/-EL2p5DTUSc_"
						},
						{
							"id": "6103174",
							"name": "TOMORROW X TOGETHER",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/TOMORROW_X_TOGETHER_001_20230113181943_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tomorrow-x-together-songs/UDH97R60FVo_"
						},
						{
							"id": "603641",
							"name": "Jonas Brothers",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Jonas_Brothers_002_20200616105406_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jonas-brothers-songs/3LJVYH-rtJY_"
						}
					]
				},
				"releaseDate": "2023-07-07",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "rp8qsdDe",
				"name": "Kiss Me More",
				"subtitle": "Doja Cat ft. SZA - Planet Her",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/kiss-me-more/AhhTQAdUc1Y",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20230721085910-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20230721085910-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20230721085910-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2021,
				"playCount": 5952767,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Amala Zandile Dlamini, David Sprecher, Roget Chahayed, Gerard A. Powell II, Carter Lang, Solana Rowe, ルーカスゴットワルド, Terry Shaddick, Stephen Kipner",
				"song": null,
				"albumId": "27978399",
				"album": "Planet Her",
				"label": "Kemosabe Records/RCA Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyJcxunMgfKkn53XApXNodRE58Ll0W+SQcE38/fJcnDqVIVwQobzAd7hw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/planet-her/0dhlIg,ymL4_",
				"duration": 208,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2021 Kemosabe Records/RCA Records",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "697799",
							"name": "Doja Cat",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Doja_Cat_20200218145042_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/doja-cat-songs/GuwOruHLpLk_"
						}
					],
					"featuredArtists": [
						{
							"id": "859314",
							"name": "SZA",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/SZA_20200218144820_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sza-songs/Ta9sLHb1eDY_"
						}
					],
					"artists": [
						{
							"id": "697800",
							"name": "Amala Zandile Dlamini",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/amala-zandile-dlamini-songs/2vDxJiTJILg_"
						},
						{
							"id": "1369835",
							"name": "David Sprecher",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/david-sprecher-songs/I0riLFoGTFM_"
						},
						{
							"id": "2165437",
							"name": "Roget Chahayed",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/roget-chahayed-songs/au3W-YdzwYA_"
						},
						{
							"id": "4382407",
							"name": "Gerard A. Powell II",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gerard-a.-powell-ii-songs/LQLDbRuJZLg_"
						},
						{
							"id": "1586306",
							"name": "Carter Lang",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/carter-lang-songs/i3YVKROQbdQ_"
						},
						{
							"id": "775315",
							"name": "Solana Rowe",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/solana-rowe-songs/s3OL4wSVwrk_"
						},
						{
							"id": "15927934",
							"name": "ルーカスゴットワルド",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/%e3%83%ab%e3%83%bc%e3%82%ab%e3%82%b9%e3%82%b4%e3%83%83%e3%83%88%e3%83%af%e3%83%ab%e3%83%89-songs/kZ9TAUD0UJg_"
						},
						{
							"id": "568888",
							"name": "Terry Shaddick",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/terry-shaddick-songs/,zu59XAAjgs_"
						},
						{
							"id": "568887",
							"name": "Stephen Kipner",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/stephen-kipner-songs/U-kPXjnLtT4_"
						},
						{
							"id": "697799",
							"name": "Doja Cat",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Doja_Cat_20200218145042_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/doja-cat-songs/GuwOruHLpLk_"
						},
						{
							"id": "859314",
							"name": "SZA",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/SZA_20200218144820_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sza-songs/Ta9sLHb1eDY_"
						},
						{
							"id": "697800",
							"name": "Amala Zandile Dlamini",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/amala-zandile-dlamini-songs/2vDxJiTJILg_"
						},
						{
							"id": "1369835",
							"name": "David Sprecher",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/david-sprecher-songs/I0riLFoGTFM_"
						},
						{
							"id": "2165437",
							"name": "Roget Chahayed",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/roget-chahayed-songs/au3W-YdzwYA_"
						},
						{
							"id": "4382407",
							"name": "Gerard A. Powell II",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/gerard-a.-powell-ii-songs/LQLDbRuJZLg_"
						},
						{
							"id": "1586306",
							"name": "Carter Lang",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/carter-lang-songs/i3YVKROQbdQ_"
						},
						{
							"id": "775315",
							"name": "Solana Rowe",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/solana-rowe-songs/s3OL4wSVwrk_"
						},
						{
							"id": "15927934",
							"name": "ルーカスゴットワルド",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/%e3%83%ab%e3%83%bc%e3%82%ab%e3%82%b9%e3%82%b4%e3%83%83%e3%83%88%e3%83%af%e3%83%ab%e3%83%89-songs/kZ9TAUD0UJg_"
						},
						{
							"id": "568888",
							"name": "Terry Shaddick",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/terry-shaddick-songs/,zu59XAAjgs_"
						},
						{
							"id": "568887",
							"name": "Stephen Kipner",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/stephen-kipner-songs/U-kPXjnLtT4_"
						}
					]
				},
				"releaseDate": "2021-06-25",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "L67KL9HY",
				"name": "I Can&#039;t Get Enough",
				"subtitle": "Benny Blanco, Tainy, Selena Gomez, J Balvin - I Can’t Get Enough",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/i-cant-get-enough/PF5cejgJf2o",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/853/I-Can-t-Get-Enough-English-2019-20190227231857-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/853/I-Can-t-Get-Enough-English-2019-20190227231857-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/853/I-Can-t-Get-Enough-English-2019-20190227231857-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2019,
				"playCount": 22134879,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Marco Masis, Benjamin Levin, Mike Sabath, Cris Chiluiza, Jose Alvaro Osorio Balvin, Jesus M. Nieves Cortez, Selena Gomez",
				"song": null,
				"albumId": "15236145",
				"album": "I Can’t Get Enough",
				"label": "NEON16/Friends Keep Secrets/Interscope Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyGuyUfZfZoBqz/1bLc1tL2IqViSJ/zi6fXd3lqDWai9JewjhZXxE3EBw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/i-can%e2%80%99t-get-enough/hZmiX7VuwpU_",
				"duration": 158,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "℗ 2019 Interscope Records",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "568563",
							"name": "Benny Blanco",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Benny_Blanco_000_20191129080344_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/benny-blanco-songs/JGPZfJ,O8pk_"
						},
						{
							"id": "1036632",
							"name": "Tainy",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Tainy_20191203071423_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tainy-songs/Q-6FV2PFy6E_"
						},
						{
							"id": "603812",
							"name": "Selena Gomez",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Selena_Gomez_002_20200226073835_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/selena-gomez-songs/x45oK7RJ2E4_"
						},
						{
							"id": "1017816",
							"name": "J Balvin",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/J_Balvin_002_20200314080220_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/j-balvin-songs/kSQKdiFEeZ8_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "593768",
							"name": "Marco Masis",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/marco-masis-songs/ZAyLW0PsFEs_"
						},
						{
							"id": "566441",
							"name": "Benjamin Levin",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/benjamin-levin-songs/2y5eaO04Ilc_"
						},
						{
							"id": "874445",
							"name": "Mike Sabath",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mike-sabath-songs/LCYv3LOCyrE_"
						},
						{
							"id": "7733961",
							"name": "Cris Chiluiza",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/cris-chiluiza-songs/TegWCf1jMM8_"
						},
						{
							"id": "740056",
							"name": "Jose Alvaro Osorio Balvin",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jose-alvaro-osorio-balvin-songs/OAynqozci,E_"
						},
						{
							"id": "4606296",
							"name": "Jesus M. Nieves Cortez",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jesus-m.-nieves-cortez-songs/14OrlYHhcrk_"
						},
						{
							"id": "603812",
							"name": "Selena Gomez",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Selena_Gomez_002_20200226073835_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/selena-gomez-songs/x45oK7RJ2E4_"
						},
						{
							"id": "568563",
							"name": "Benny Blanco",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Benny_Blanco_000_20191129080344_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/benny-blanco-songs/JGPZfJ,O8pk_"
						},
						{
							"id": "1036632",
							"name": "Tainy",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Tainy_20191203071423_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/tainy-songs/Q-6FV2PFy6E_"
						},
						{
							"id": "603812",
							"name": "Selena Gomez",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Selena_Gomez_002_20200226073835_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/selena-gomez-songs/x45oK7RJ2E4_"
						},
						{
							"id": "1017816",
							"name": "J Balvin",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/J_Balvin_002_20200314080220_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/j-balvin-songs/kSQKdiFEeZ8_"
						},
						{
							"id": "593768",
							"name": "Marco Masis",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/marco-masis-songs/ZAyLW0PsFEs_"
						},
						{
							"id": "566441",
							"name": "Benjamin Levin",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/benjamin-levin-songs/2y5eaO04Ilc_"
						},
						{
							"id": "874445",
							"name": "Mike Sabath",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/mike-sabath-songs/LCYv3LOCyrE_"
						},
						{
							"id": "7733961",
							"name": "Cris Chiluiza",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/cris-chiluiza-songs/TegWCf1jMM8_"
						},
						{
							"id": "740056",
							"name": "Jose Alvaro Osorio Balvin",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jose-alvaro-osorio-balvin-songs/OAynqozci,E_"
						},
						{
							"id": "4606296",
							"name": "Jesus M. Nieves Cortez",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/jesus-m.-nieves-cortez-songs/14OrlYHhcrk_"
						},
						{
							"id": "603812",
							"name": "Selena Gomez",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Selena_Gomez_002_20200226073835_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/selena-gomez-songs/x45oK7RJ2E4_"
						}
					]
				},
				"releaseDate": "2019-02-28",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "2IuQsex6",
				"name": "Levitating (feat. DaBaby)",
				"subtitle": "Dua Lipa ft. DaBaby - Levitating (feat. DaBaby)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/levitating-feat.-dababy/QiEeYAdVTwU",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/651/Levitating-feat-DaBaby--English-2020-20200928114801-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/651/Levitating-feat-DaBaby--English-2020-20200928114801-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/651/Levitating-feat-DaBaby--English-2020-20200928114801-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2020,
				"playCount": 18599579,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "22765589",
				"album": "Levitating (feat. DaBaby)",
				"label": "Warner Records",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyomLchZkStF1/hSyKx1NKBRB6euxXKTpCFcD3CWvufjblUgOCNBKochw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/levitating-feat.-dababy/9WB4qrU9Stw_",
				"duration": 203,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "Under exclusive license to Warner Records UK, a division of Warner Music UK Limited, ℗ 2020  Dua Lipa Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "1274170",
							"name": "Dua Lipa",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Dua_Lipa_003_20200226102459_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dua-lipa-songs/r-OWIKgpX2I_"
						}
					],
					"featuredArtists": [
						{
							"id": "2436622",
							"name": "DaBaby",
							"role": "featured_artists",
							"image": "https://c.saavncdn.com/artists/DaBaby_003_20230627114405_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dababy-songs/woXlnIzw1zU_"
						}
					],
					"artists": [
						{
							"id": "1274170",
							"name": "Dua Lipa",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Dua_Lipa_003_20200226102459_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dua-lipa-songs/r-OWIKgpX2I_"
						}
					]
				},
				"releaseDate": "2020-10-01",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "G7LSnqvV",
				"name": "Hey, Soul Sister",
				"subtitle": "Pat Monahan, Train - Save Me, San Francisco",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/hey-soul-sister/N18nYhpBQWU",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/621/Save-Me-San-Francisco-2009-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/621/Save-Me-San-Francisco-2009-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/621/Save-Me-San-Francisco-2009-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2009,
				"playCount": 1052273,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Amund Björklund, Espen Lind, Pat Monahan",
				"song": null,
				"albumId": "1102219",
				"album": "Save Me, San Francisco",
				"label": "Sony Music",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyCXFy+523+vtCCx0P6SDfaCaNub2jaTxlaep4AaTdXSkhfniYnSrH9Bw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/save-me-san-francisco/QcgxLOLITsg_",
				"duration": 216,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "©  2009 Sony Music",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "528410",
							"name": "Pat Monahan",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/689/Her-Eyes-2007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pat-monahan-songs/WGFFOu38mrM_"
						},
						{
							"id": "528413",
							"name": "Train",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/317/Call-Me-Sir-English-2018-20180517192441-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/train-songs/3LjxiCoeoLg_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "566891",
							"name": "Amund Björklund",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/amund-bjorklund-songs/X2Ekp5CcmtE_"
						},
						{
							"id": "566892",
							"name": "Espen Lind",
							"role": "music",
							"image": "https://c.saavncdn.com/724/This-Is-Pop-Music-2009-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/espen-lind-songs/lUHJC,h5xYw_"
						},
						{
							"id": "528410",
							"name": "Pat Monahan",
							"role": "music",
							"image": "https://c.saavncdn.com/689/Her-Eyes-2007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pat-monahan-songs/WGFFOu38mrM_"
						},
						{
							"id": "528410",
							"name": "Pat Monahan",
							"role": "singer",
							"image": "https://c.saavncdn.com/689/Her-Eyes-2007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pat-monahan-songs/WGFFOu38mrM_"
						},
						{
							"id": "528413",
							"name": "Train",
							"role": "singer",
							"image": "https://c.saavncdn.com/317/Call-Me-Sir-English-2018-20180517192441-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/train-songs/3LjxiCoeoLg_"
						},
						{
							"id": "566891",
							"name": "Amund Björklund",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/amund-bjorklund-songs/X2Ekp5CcmtE_"
						},
						{
							"id": "566892",
							"name": "Espen Lind",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/724/This-Is-Pop-Music-2009-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/espen-lind-songs/lUHJC,h5xYw_"
						},
						{
							"id": "528410",
							"name": "Pat Monahan",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/689/Her-Eyes-2007-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pat-monahan-songs/WGFFOu38mrM_"
						}
					]
				},
				"releaseDate": "2009-10-23",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "1zRGHMMB",
				"name": "Feels",
				"subtitle": "Calvin Harris ft. Pharrell Williams, Katy Perry, Big Sean, Pharrell Williams, Katy Perry &amp; Big Sean - Feels",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/feels/QRI5djx9enE",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/918/Feels-English-2017-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/918/Feels-English-2017-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/918/Feels-English-2017-500x500.jpg"
					}
				],
				"language": "english",
				"year": 2017,
				"playCount": 10779250,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Pharrell Williams, Katy Perry, Adam Wiles, Brittany Hazzard, Sean Anderson",
				"song": null,
				"albumId": "11005030",
				"album": "Feels",
				"label": "Columbia",
				"origin": "playlist",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyP9acfo354yybbzj4IkfRs4tWE45mMnwWCl04xU02qzS5XNjvs1Vvmhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/feels/OuMkPhCrWNM_",
				"duration": 223,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "(P) 2017 Sony Music Entertainment UK Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "565985",
							"name": "Calvin Harris",
							"role": "primary_artists",
							"image": "http://c.saavncdn.com/artists/Calvin_Harris_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/calvin-harris-songs/QMnR-zr4Bng_"
						}
					],
					"featuredArtists": [
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "featured_artists",
							"image": "http://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "565740",
							"name": "Katy Perry",
							"role": "featured_artists",
							"image": "http://c.saavncdn.com/artists/Katy_Perry_004_20200616105931_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/katy-perry-songs/Q8k4MniH10k_"
						},
						{
							"id": "609221",
							"name": "Big Sean",
							"role": "featured_artists",
							"image": "http://c.saavncdn.com/artists/Big_Sean_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/big-sean-songs/nKHIWhJuVzs_"
						},
						{
							"id": "3508793",
							"name": "Pharrell Williams",
							"role": "featured_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/47qOHtBlYeM_"
						},
						{
							"id": "",
							"name": "Katy Perry &amp; Big Sean",
							"role": "featured_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/katy-perry-big-sean-songs/"
						}
					],
					"artists": [
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "music",
							"image": "http://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "565740",
							"name": "Katy Perry",
							"role": "music",
							"image": "http://c.saavncdn.com/artists/Katy_Perry_004_20200616105931_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/katy-perry-songs/Q8k4MniH10k_"
						},
						{
							"id": "566765",
							"name": "Adam Wiles",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-wiles-songs/1RLko-qnkQ0_"
						},
						{
							"id": "1148615",
							"name": "Brittany Hazzard",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brittany-hazzard-songs/bJv0dWdz9Lo_"
						},
						{
							"id": "578291",
							"name": "Sean Anderson",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sean-anderson-songs/AXzwdEdK1n8_"
						},
						{
							"id": "565985",
							"name": "Calvin Harris",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Calvin_Harris_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/calvin-harris-songs/QMnR-zr4Bng_"
						},
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "565740",
							"name": "Katy Perry",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Katy_Perry_004_20200616105931_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/katy-perry-songs/Q8k4MniH10k_"
						},
						{
							"id": "609221",
							"name": "Big Sean",
							"role": "singer",
							"image": "http://c.saavncdn.com/artists/Big_Sean_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/big-sean-songs/nKHIWhJuVzs_"
						},
						{
							"id": "522132",
							"name": "Pharrell Williams",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/artists/Pharrell_Williams_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/pharrell-williams-songs/UKgAQUvEyjc_"
						},
						{
							"id": "565740",
							"name": "Katy Perry",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/artists/Katy_Perry_004_20200616105931_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/katy-perry-songs/Q8k4MniH10k_"
						},
						{
							"id": "566765",
							"name": "Adam Wiles",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/adam-wiles-songs/1RLko-qnkQ0_"
						},
						{
							"id": "1148615",
							"name": "Brittany Hazzard",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/brittany-hazzard-songs/bJv0dWdz9Lo_"
						},
						{
							"id": "578291",
							"name": "Sean Anderson",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sean-anderson-songs/AXzwdEdK1n8_"
						}
					]
				},
				"releaseDate": "2017-06-16",
				"trillerAvailable": false,
				"lyricsId": null
			}
		]
	}
}
```

+++

|            Query Parameter            | Description |                Required                |
| :-----------------------------------: | :---------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] | Playlist ID | [!badge variant="primary" text="True"] |
