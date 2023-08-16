---
order: 700
icon: play
---

# Search Songs

Search songs by query.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/songs?query=ram+siya+ram&page=1&limit=2
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/songs?query=ram+siya+ram&page=1&limit=2' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Search results for songs fetched successfully!",
	"data": {
		"total": 1432,
		"start": 1,
		"results": [
			{
				"id": "HLulXlir",
				"name": "Ram Siya Ram",
				"subtitle": "Sachet Tandon - Ram Siya Ram",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-500x500.jpg"
					}
				],
				"language": "hindi",
				"year": 2021,
				"playCount": 61252424,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Poonam Thakkar",
				"song": null,
				"albumId": "25500145",
				"album": "Ram Siya Ram",
				"label": "",
				"origin": "search",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyYJ+5/mMmphgKGwkNDtSkT/MwZwumWrWHz1fM9uHMbp+aUIGETZOvHRw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
				"duration": 265,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "false",
				"hasLyrics": true,
				"lyricsSnippet": "Lyrics",
				"starred": false,
				"copyrightText": "℗ 2021 Super Cassettes Industries Private Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "3623110",
							"name": "Sachet Tandon",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "9521286",
							"name": "Poonam Thakkar",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/poonam-thakkar-songs/K9zFVB-gAME_"
						},
						{
							"id": "3623110",
							"name": "Sachet Tandon",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
						},
						{
							"id": "461011",
							"name": "Shabbir Ahmed",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/artists/Shabbir_Ahmed_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/shabbir-ahmed-songs/CUgZ-dg1Y9w_"
						}
					]
				},
				"releaseDate": "",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "dBM1AsPC",
				"name": "Ram Siya Ram",
				"subtitle": "Sachet-Parampara, Sachet Tandon, Parampara Tandon - Adipurush",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/ram-siya-ram/FComADVDZ3A",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-500x500.jpg"
					}
				],
				"language": "hindi",
				"year": 2023,
				"playCount": 20339961,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Sachet-Parampara",
				"song": null,
				"albumId": "45770853",
				"album": "Adipurush",
				"label": "",
				"origin": "search",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyE54drfnt4WGgljKLFTJQee11y7OelHvDKuilTC7W+h6me5BHNGfR/hw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/adipurush/YZ03kAOU8ow_",
				"duration": 230,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "false",
				"hasLyrics": true,
				"lyricsSnippet": "Ho Janam Janam Ki Khoj Bataye",
				"starred": false,
				"copyrightText": "℗ 2023 Super Cassettes Industries Private Limited",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "3623112",
							"name": "Sachet-Parampara",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Sachet-Parampara_20190221095720_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-parampara-songs/-01nNmS1dCs_"
						},
						{
							"id": "3623110",
							"name": "Sachet Tandon",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
						},
						{
							"id": "3623111",
							"name": "Parampara Tandon",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Parampara_Thakur_20191130070743_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/parampara-tandon-songs/cJO7csRHiSM_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "3623112",
							"name": "Sachet-Parampara",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Sachet-Parampara_20190221095720_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-parampara-songs/-01nNmS1dCs_"
						},
						{
							"id": "3623112",
							"name": "Sachet-Parampara",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Sachet-Parampara_20190221095720_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-parampara-songs/-01nNmS1dCs_"
						},
						{
							"id": "3623110",
							"name": "Sachet Tandon",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
						},
						{
							"id": "3623111",
							"name": "Parampara Tandon",
							"role": "singer",
							"image": "https://c.saavncdn.com/artists/Parampara_Thakur_20191130070743_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/parampara-tandon-songs/cJO7csRHiSM_"
						},
						{
							"id": "473441",
							"name": "Manoj Muntashir",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/259/Episode-1-Dekhte-Dekhte-From-Muntashir-Ki-Diary-Se--Hindi-2019-20190902135108-150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/manoj-muntashir-songs/eaiDjU0BhyA_"
						}
					]
				},
				"releaseDate": "",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "FNk9HwWp",
				"name": "Ram Siya Ram",
				"subtitle": "Rakesh Kala - Ram Siya Ram",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/ram-siya-ram/NiYACDxHYEM",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-500x500.jpg"
					}
				],
				"language": "hindi",
				"year": 2021,
				"playCount": 3778297,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Rakesh Kala, V. Jai",
				"song": null,
				"albumId": "24471788",
				"album": "Ram Siya Ram",
				"label": "Fatafat Digital",
				"origin": "search",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDycf/J8DQ0jQc03i9O+ymVjiENdXF7mZrw39OwtmVeowmcZFUY5hOhHRw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram/gz,aRUzsOs0_",
				"duration": 596,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "false",
				"hasLyrics": true,
				"lyricsSnippet": "ram, siya ram, siya ram, jai-jai ram",
				"starred": false,
				"copyrightText": "℗ Fatafat Digital | ST",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "459629",
							"name": "Rakesh Kala",
							"role": "primary_artists",
							"image": "https://c.saavncdn.com/artists/Rakesh_Kala_20191215152225_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/rakesh-kala-songs/NSmfJzqVn,c_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "459629",
							"name": "Rakesh Kala",
							"role": "music",
							"image": "https://c.saavncdn.com/artists/Rakesh_Kala_20191215152225_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/rakesh-kala-songs/NSmfJzqVn,c_"
						},
						{
							"id": "8575317",
							"name": "V. Jai",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/v.-jai-songs/p2DyxccB5Y8_"
						},
						{
							"id": "455090",
							"name": "Traditional",
							"role": "lyricist",
							"image": "https://c.saavncdn.com/186/Krishna-Bhajans-Hindi-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/traditional-songs/1W8p9iMeXm4_"
						}
					]
				},
				"releaseDate": "",
				"trillerAvailable": false,
				"lyricsId": ""
			},
			{
				"id": "DQtrXQP0",
				"name": "Ram Siya Ram (Slowed &amp; Reverbed)",
				"subtitle": "Dalip Shekhawat - Ram Siya Ram (Slowed &amp; Reverbed)",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/ram-siya-ram-slowed-reverbed/NDkfQyxhZwM",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-500x500.jpg"
					}
				],
				"language": "sanskrit",
				"year": 2022,
				"playCount": 212226,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "",
				"song": null,
				"albumId": "41250229",
				"album": "Ram Siya Ram (Slowed &amp; Reverbed)",
				"label": "Dalip Shekhawat",
				"origin": "search",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyXHXRTZEITEj06eai0MZ1IKvtUuzoTFwheQSeaS92/ShYvwCotzOaNhw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram-slowed-reverbed/vLMTlP1cDP0_",
				"duration": 350,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "false",
				"hasLyrics": false,
				"lyricsSnippet": "",
				"starred": false,
				"copyrightText": "© 2022 Dalip Shekhawat",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "14520114",
							"name": "Dalip Shekhawat",
							"role": "primary_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dalip-shekhawat-songs/-8hMWnyPq90_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "14520114",
							"name": "Dalip Shekhawat",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dalip-shekhawat-songs/-8hMWnyPq90_"
						},
						{
							"id": "14520114",
							"name": "Dalip Shekhawat",
							"role": "lyricist",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/dalip-shekhawat-songs/-8hMWnyPq90_"
						}
					]
				},
				"releaseDate": "",
				"trillerAvailable": false,
				"lyricsId": null
			},
			{
				"id": "CrPBg4DF",
				"name": "Ram Siya Ram",
				"subtitle": "Harshit Joshi - Ram Siya Ram",
				"type": "song",
				"url": "https://www.jiosaavn.com/song/ram-siya-ram/Mxo7cxMEc3U",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/922/Ram-Siya-Ram-Hindi-2022-20221025105153-50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/922/Ram-Siya-Ram-Hindi-2022-20221025105153-150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/922/Ram-Siya-Ram-Hindi-2022-20221025105153-500x500.jpg"
					}
				],
				"language": "hindi",
				"year": 2022,
				"playCount": 318698,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Harshit Joshi",
				"song": null,
				"albumId": "39363379",
				"album": "Ram Siya Ram",
				"label": "Namo Music",
				"origin": "search",
				"isDolbyContent": false,
				"320kbps": true,
				"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyW562PgVsKePglBdDRbdHz6ZNcvtdeUG7wf67qrFO1/nxKH+9Xwpdthw7tS9a8Gtq",
				"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram/IrMJZxqWlOI_",
				"duration": 65,
				"rights": {
					"code": "0",
					"cacheable": "true",
					"delete_cached_object": "false",
					"reason": ""
				},
				"cacheState": "false",
				"hasLyrics": true,
				"lyricsSnippet": "राम, सिया-राम, सिया-राम, जय-जय राम",
				"starred": false,
				"copyrightText": "© 2022 NaMo Music",
				"artistMap": {
					"primaryArtists": [
						{
							"id": "10641288",
							"name": "Harshit Joshi",
							"role": "primary_artists",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harshit-joshi-songs/EFfGpNkfcGY_"
						}
					],
					"featuredArtists": [],
					"artists": [
						{
							"id": "10641288",
							"name": "Harshit Joshi",
							"role": "music",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harshit-joshi-songs/EFfGpNkfcGY_"
						},
						{
							"id": "10641288",
							"name": "Harshit Joshi",
							"role": "singer",
							"image": "",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/harshit-joshi-songs/EFfGpNkfcGY_"
						},
						{
							"id": "455090",
							"name": "Traditional",
							"role": "lyricist",
							"image": "http://c.saavncdn.com/186/Krishna-Bhajans-Hindi-2010-150x150_150x150.jpg",
							"type": "artist",
							"url": "https://www.jiosaavn.com/artist/traditional-songs/1W8p9iMeXm4_"
						}
					]
				},
				"releaseDate": "",
				"trillerAvailable": false,
				"lyricsId": ""
			}
		]
	}
}
```

+++

| Query Parameter                          | Description             | Default | Required                                |
| ---------------------------------------- | ----------------------- | ------- | --------------------------------------- |
| [!badge variant="contrast" text="query"] | Search query            | -       | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="page"]  | Current page            | 1       | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="limit"] | limit number of results | 10      | [!badge variant="primary" text="False"] |
