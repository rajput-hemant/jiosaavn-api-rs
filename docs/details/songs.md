---
order: 100
icon: play
---

# Song Details

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Song details by song ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/song?id=HLulXlir
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/song?id=HLulXlir' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Song details fetched successfully!",
	"data": [
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
			"playCount": 61469765,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Poonam Thakkar",
			"song": null,
			"albumId": "25500145",
			"album": "Ram Siya Ram",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": [
				{
					"quality": "12kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_21.mp4"
				},
				{
					"quality": "48kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_48.mp4"
				},
				{
					"quality": "96kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_96.mp4"
				},
				{
					"quality": "160kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_160.mp4"
				},
				{
					"quality": "320kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_320.mp4"
				}
			],
			"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
			"duration": 265,
			"rights": {
				"code": "1",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": "Pro Only"
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
			"releaseDate": "2021-02-22",
			"trillerAvailable": false,
			"lyricsId": ""
		}
	]
}
```

+++

|            Query Parameter            | Description |                Required                |
| :-----------------------------------: | :---------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] |   Song ID   | [!badge variant="primary" text="True"] |

## Song details by link

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/song?link=https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/song?link=https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Song details fetched successfully!",
	"data": [
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
			"playCount": 61469765,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Poonam Thakkar",
			"song": null,
			"albumId": "25500145",
			"album": "Ram Siya Ram",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": [
				{
					"quality": "12kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_21.mp4"
				},
				{
					"quality": "48kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_48.mp4"
				},
				{
					"quality": "96kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_96.mp4"
				},
				{
					"quality": "160kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_160.mp4"
				},
				{
					"quality": "320kbps",
					"link": "https://aac.saavncdn.com/215/5a6134d5860b9b47cc9fb2e058e22a18_320.mp4"
				}
			],
			"albumUrl": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
			"duration": 265,
			"rights": {
				"code": "1",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": "Pro Only"
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
			"releaseDate": "2021-02-22",
			"trillerAvailable": false,
			"lyricsId": ""
		}
	]
}
```

+++

|             Query Parameter             |              Description              |                Required                |
| :-------------------------------------: | :-----------------------------------: | :------------------------------------: |
| [!badge variant="contrast" text="link"] | Song link from <https://jiosaavn.com> | [!badge variant="primary" text="True"] |

## Song Recommendations by song ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/song/recommendations?id=HLulXlir
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/song/recommendations?id=HLulXlir' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Successfully fetched song recommendations",
	"data": [
		{
			"id": "095cGCaU",
			"name": "Radhe Radhe Barsane Wali Radhe",
			"subtitle": "Bihari Ji - Radhe Radhe Barsane Wali Radhe",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/radhe-radhe-barsane-wali-radhe/QFFeUjNzVmY",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/035/Radhe-Radhe-Barsane-Wali-Radhe-Hindi-2022-20220913102408-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/035/Radhe-Radhe-Barsane-Wali-Radhe-Hindi-2022-20220913102408-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/035/Radhe-Radhe-Barsane-Wali-Radhe-Hindi-2022-20220913102408-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2022,
			"playCount": 4806631,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Vipul",
			"song": null,
			"albumId": "38162579",
			"album": "Radhe Radhe Barsane Wali Radhe",
			"label": "Vipul Music Company",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy1qxHA5y/rl9E7mNT8/0QREMEoRlStp8/LDedc7er+Z5DLnzNj0Ng7Bw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/radhe-radhe-barsane-wali-radhe/iwbvK9cuukw_",
			"duration": 2693,
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
			"copyrightText": "© 2022 Vipul Music Company",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "459211",
						"name": "Bihari Ji",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/bihari-ji-songs/P75Wn4f6viI_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "504034",
						"name": "Vipul",
						"role": "music",
						"image": "https://c.saavncdn.com/365/Yaar-Beli-Punjabi-2018-20180328160527-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/vipul-songs/3zxJph5,tXo_"
					},
					{
						"id": "459211",
						"name": "Bihari Ji",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/bihari-ji-songs/P75Wn4f6viI_"
					},
					{
						"id": "504034",
						"name": "Vipul",
						"role": "lyricist",
						"image": "https://c.saavncdn.com/365/Yaar-Beli-Punjabi-2018-20180328160527-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/vipul-songs/3zxJph5,tXo_"
					}
				]
			},
			"releaseDate": "2022-09-12",
			"trillerAvailable": false,
			"lyricsId": null
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
			"playCount": 3787412,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Rakesh Kala, V. Jai",
			"song": null,
			"albumId": "24471788",
			"album": "Ram Siya Ram",
			"label": "Fatafat Digital",
			"origin": "none",
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
			"releaseDate": "2021-01-05",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "T0qAr4wo",
			"name": "Radhe Braj Jan Man Sukhkari",
			"subtitle": "Devi Neha Saraswat - Radhe Braj Jan Man Sukhkari",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/radhe-braj-jan-man-sukhkari/JFgacAYEQFw",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/141/Radhe-Braj-Jan-Man-Sukhkari-Hindi-2022-20220111165621-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/141/Radhe-Braj-Jan-Man-Sukhkari-Hindi-2022-20220111165621-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/141/Radhe-Braj-Jan-Man-Sukhkari-Hindi-2022-20220111165621-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2022,
			"playCount": 5276588,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Devi Neha Saraswat, Lovely Sharma",
			"song": null,
			"albumId": "32093449",
			"album": "Radhe Braj Jan Man Sukhkari",
			"label": "Supertone Digital",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDydEL6gtCXujddDvyraWZRz9yzeurxphfCs4biJkTINkkQVMNzA2O/jRw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/radhe-braj-jan-man-sukhkari/8wA12fWUKiQ_",
			"duration": 423,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "raadhe shyaam, shyama, shyaam",
			"starred": false,
			"copyrightText": "℗ 2021 Supertone Digital",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "11618518",
						"name": "Devi Neha Saraswat",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/devi-neha-saraswat-songs/tRrfuLcxNzI_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "11618518",
						"name": "Devi Neha Saraswat",
						"role": "music",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/devi-neha-saraswat-songs/tRrfuLcxNzI_"
					},
					{
						"id": "459196",
						"name": "Lovely Sharma",
						"role": "music",
						"image": "https://c.saavncdn.com/055/Kaanch-Kasaili-2005-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/lovely-sharma-songs/ensCZnUIxR8_"
					},
					{
						"id": "12209156",
						"name": "Narayana Maharaja",
						"role": "lyricist",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/narayana-maharaja-songs/5GGgwVxIMGI_"
					}
				]
			},
			"releaseDate": "2022-01-01",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "V_vt-OZ8",
			"name": "Shri Krishna Govind Hare Murari",
			"subtitle": "Jubin Nautiyal - Shri Krishna Govind Hare Murari",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/shri-krishna-govind-hare-murari/JjcdRVl-bQs",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/021/Shri-Krishna-Govind-Hare-Murari-Hindi-2020-20200806224005-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/021/Shri-Krishna-Govind-Hare-Murari-Hindi-2020-20200806224005-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/021/Shri-Krishna-Govind-Hare-Murari-Hindi-2020-20200806224005-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2020,
			"playCount": 19968338,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Raaj Aashoo",
			"song": null,
			"albumId": "21902776",
			"album": "Shri Krishna Govind Hare Murari",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyQIFzBIC5HMVSg61ZjFHevLiZz1kh8nuJK5XcHaYsKWSpjYb/TOUScRw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/shri-krishna-govind-hare-murari/pq8zZ7GVLQg_",
			"duration": 153,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "devaki nandan tumko vandan rakhte sabaki laaj",
			"starred": false,
			"copyrightText": "℗ 2020 Super Cassettes Industries Private Limited",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "881158",
						"name": "Jubin Nautiyal",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "466955",
						"name": "Raaj Aashoo",
						"role": "music",
						"image": "https://c.saavncdn.com/298/So-Gaya-Yeh-Jahan-From-Bypass-Road--Hindi-2019-20191010075008-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/raaj-aashoo-songs/KqvThBgSv7A_"
					},
					{
						"id": "881158",
						"name": "Jubin Nautiyal",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
					},
					{
						"id": "6926638",
						"name": "Murali Agarwal",
						"role": "lyricist",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/murali-agarwal-songs/xyKkvW0EUVQ_"
					}
				]
			},
			"releaseDate": "2020-08-07",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "4chfVGeU",
			"name": "Lagan Tumse Laga Baithe",
			"subtitle": "Kavita Raam - Krishna Bhajan - Lagan Tumse Laga Baithe",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/lagan-tumse-laga-baithe/RAsDVyJ3UmY",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/735/Krishna-Bhajan-Lagan-Tumse-Laga-Baithe-Hindi-2021-20210227002702-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/735/Krishna-Bhajan-Lagan-Tumse-Laga-Baithe-Hindi-2021-20210227002702-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/735/Krishna-Bhajan-Lagan-Tumse-Laga-Baithe-Hindi-2021-20210227002702-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2021,
			"playCount": 24214566,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Navin, Manish",
			"song": null,
			"albumId": "25605376",
			"album": "Krishna Bhajan - Lagan Tumse Laga Baithe",
			"label": "Music Nova",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyIAZ31xv3Fn7oft8yk+h7vYcgjfSx58aGnJEe/G8ZPw6rHAfiKUU/xBw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/krishna-bhajan-lagan-tumse-laga-baithe/o21Jcy8VKZg_",
			"duration": 357,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "जो होगा देखा जायेगा",
			"starred": false,
			"copyrightText": "© 2021 Music Nova",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "5204884",
						"name": "Kavita Raam",
						"role": "primary_artists",
						"image": "http://c.saavncdn.com/artists/Kavita_Raam_002_20191216103837_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kavita-raam-songs/L,lIOTN5MiQ_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "484041",
						"name": "Navin",
						"role": "music",
						"image": "http://c.saavncdn.com/212/Naalu-Perukku-Nalladhuna-Edhuvum-Thapilla-Tamil-2016-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-songs/YC1s79jeC8I_"
					},
					{
						"id": "464570",
						"name": "Manish",
						"role": "music",
						"image": "http://c.saavncdn.com/085/Baba-Ro-Melo-Aa-Gayo-Rajasthani-2017-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/manish-songs/xutLLs0piOs_"
					},
					{
						"id": "5204884",
						"name": "Kavita Raam",
						"role": "singer",
						"image": "http://c.saavncdn.com/artists/Kavita_Raam_002_20191216103837_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kavita-raam-songs/L,lIOTN5MiQ_"
					}
				]
			},
			"releaseDate": "2021-02-25",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "RyHP7Ta6",
			"name": "Shyama Aan Baso Vrindawan Mein",
			"subtitle": "Upasana Mehta - Shyama Aan Baso Vrindawan Mein",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/shyama-aan-baso-vrindawan-mein/IhEjYUNkVgU",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/220/Shyama-Aan-Baso-Vrindawan-Mein-Hindi-2021-20210420002646-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/220/Shyama-Aan-Baso-Vrindawan-Mein-Hindi-2021-20210420002646-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/220/Shyama-Aan-Baso-Vrindawan-Mein-Hindi-2021-20210420002646-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2021,
			"playCount": 2119465,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Binny Narang",
			"song": null,
			"albumId": "26654599",
			"album": "Shyama Aan Baso Vrindawan Mein",
			"label": "Bhakti Sadhna",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy6KHwC9z/0LfX4sbjlU8moeGjahV7aHkHUBuzQgN1rX0xIF5D4MS+VBw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/shyama-aan-baso-vrindawan-mein/wKNJA1E2UoM_",
			"duration": 539,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "meri umr biit gayi gokul mein",
			"starred": false,
			"copyrightText": "© 2021 Bhakti Sadhna",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "8377126",
						"name": "Upasana Mehta",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/upasana-mehta-songs/MiW60hV8raw_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "4260159",
						"name": "Binny Narang",
						"role": "music",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/binny-narang-songs/QkNaHN1UOUA_"
					},
					{
						"id": "8377126",
						"name": "Upasana Mehta",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/upasana-mehta-songs/MiW60hV8raw_"
					}
				]
			},
			"releaseDate": "2021-04-17",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "NANXt7xX",
			"name": "Mangal Bhawan Amangal Haari",
			"subtitle": "Amandeep Singh - Mangal Bhawan Amangal Haari",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/mangal-bhawan-amangal-haari/PiklaQAHT2s",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/572/Mangal-Bhawan-Amangal-Haari-Hindi-2020-20200907055635-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/572/Mangal-Bhawan-Amangal-Haari-Hindi-2020-20200907055635-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/572/Mangal-Bhawan-Amangal-Haari-Hindi-2020-20200907055635-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2020,
			"playCount": 2398622,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Amandeep Singh",
			"song": null,
			"albumId": "22339081",
			"album": "Mangal Bhawan Amangal Haari",
			"label": "Shaj Production",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyhebzCP/NvbFEKH3pdqKAdPS4Oe/FzNJX64JqtnCyEw9oRELTAq2/qRw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/mangal-bhawan-amangal-haari/5TFD4jjLGUQ_",
			"duration": 259,
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
			"copyrightText": "℗ 2020 Shaj Production",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "496863",
						"name": "Amandeep Singh",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/571/Shiv-Shambhu-Mera-Bhola-Bhala-Hindi-2019-20190813183950-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/amandeep-singh-songs/Qj9ShWpkcXc_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "496863",
						"name": "Amandeep Singh",
						"role": "music",
						"image": "https://c.saavncdn.com/571/Shiv-Shambhu-Mera-Bhola-Bhala-Hindi-2019-20190813183950-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/amandeep-singh-songs/Qj9ShWpkcXc_"
					},
					{
						"id": "496863",
						"name": "Amandeep Singh",
						"role": "singer",
						"image": "https://c.saavncdn.com/571/Shiv-Shambhu-Mera-Bhola-Bhala-Hindi-2019-20190813183950-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/amandeep-singh-songs/Qj9ShWpkcXc_"
					}
				]
			},
			"releaseDate": "2020-09-04",
			"trillerAvailable": false,
			"lyricsId": null
		},
		{
			"id": "t5DjvByS",
			"name": "Pal Pal Dil Ke Paas - Title Track",
			"subtitle": "Arijit Singh, Parampara Tandon - Pal Pal Dil Ke Paas",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/pal-pal-dil-ke-paas-title-track/BF0vWwJyTmA",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/328/Pal-Pal-Dil-Ke-Paas-Hindi-2019-20200420150444-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/328/Pal-Pal-Dil-Ke-Paas-Hindi-2019-20200420150444-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/328/Pal-Pal-Dil-Ke-Paas-Hindi-2019-20200420150444-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2019,
			"playCount": 411136644,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Sachet-Parampara, Rishi Rich",
			"song": null,
			"albumId": "16991156",
			"album": "Pal Pal Dil Ke Paas",
			"label": "Zee Music Co.",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyY/4iBUPaCkzstsq5DnX8nsA2By6/GugPDYzEGpuPzKTM2l1/oaLRChw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/pal-pal-dil-ke-paas/F35tzads,xY_",
			"duration": 254,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "Seene se tere sarr ko laga ke",
			"starred": false,
			"copyrightText": "© 2019 Zee Music Company",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "459320",
						"name": "Arijit Singh",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
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
						"id": "472306",
						"name": "Rishi Rich",
						"role": "music",
						"image": "https://c.saavncdn.com/artists/Rishi_Rich_20180614102841_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/rishi-rich-songs/Ra-3uyqjWyo_"
					},
					{
						"id": "459320",
						"name": "Arijit Singh",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
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
						"id": "677149",
						"name": "Siddharth-Garima",
						"role": "lyricist",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/siddharth-garima-songs/ftVD5tjFzrs_"
					},
					{
						"id": "6832267",
						"name": "Akash Dhar",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/akash-dhar-songs/gFKVechi48Q_"
					},
					{
						"id": "455288",
						"name": "Sachin Khedekar",
						"role": "starring",
						"image": "https://c.saavncdn.com/artists/Sachin_Khedekar_000_20230809115814_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/sachin-khedekar-songs/aTIedJrzPVc_"
					},
					{
						"id": "6874614",
						"name": "Megna Malik",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/megna-malik-songs/Q2WBJPftJ7A_"
					},
					{
						"id": "670855",
						"name": "Karan Deol",
						"role": "starring",
						"image": "https://c.saavncdn.com/artists/Karan_Deol_000_20220608064833_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/karan-deol-songs/3gVba7vDSgg_"
					},
					{
						"id": "464682",
						"name": "Simone Singh",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/simone-singh-songs/fkdXpkeotgI_"
					},
					{
						"id": "457047",
						"name": "Kamini Khanna",
						"role": "starring",
						"image": "https://c.saavncdn.com/589/Aaj-Hai-Jagrata-Hindi-1999-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kamini-khanna-songs/zGHYUA3u5Vo_"
					},
					{
						"id": "6832265",
						"name": "Sahher Bambba",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/sahher-bambba-songs/P3XVp4puxVQ_"
					},
					{
						"id": "6832266",
						"name": "Aakash Ahuja",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/aakash-ahuja-songs/BpWPL,9JrTo_"
					}
				]
			},
			"releaseDate": "2019-08-29",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "yc5XXdke",
			"name": "Thoda Thoda Pyaar",
			"subtitle": "Stebin Ben - Thoda Thoda Pyaar",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/thoda-thoda-pyaar/CQteaSxUXFY",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/959/Thoda-Thoda-Pyaar-Hindi-2021-20210212084501-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/959/Thoda-Thoda-Pyaar-Hindi-2021-20210212084501-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/959/Thoda-Thoda-Pyaar-Hindi-2021-20210212084501-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2021,
			"playCount": 263005436,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Nilesh Ahuja",
			"song": null,
			"albumId": "25291894",
			"album": "Thoda Thoda Pyaar",
			"label": "Zee Music Co.",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDylSAYC6EK37BR4QNJ4z4G+W4xu5NCkQWldsIEuFLgzfMUnpSJ96c6oBw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/thoda-thoda-pyaar/7F6S84kDvPo_",
			"duration": 244,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "ki thoda-thoda pyaar hua  tum se",
			"starred": false,
			"copyrightText": "© 2021 Zee Music Company",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "4670197",
						"name": "Stebin Ben",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Stebin_Ben_004_20200930091339_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/stebin-ben-songs/cIgvj9lx6Dc_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "2029028",
						"name": "Nilesh Ahuja",
						"role": "music",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/nilesh-ahuja-songs/GrTspM8TKSs_"
					},
					{
						"id": "4670197",
						"name": "Stebin Ben",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Stebin_Ben_004_20200930091339_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/stebin-ben-songs/cIgvj9lx6Dc_"
					},
					{
						"id": "455665",
						"name": "Kumaar",
						"role": "lyricist",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kumaar-songs/jXhf,IMIGGs_"
					},
					{
						"id": "682930",
						"name": "Sidharth Malhotra",
						"role": "starring",
						"image": "https://c.saavncdn.com/artists/Sidharth_Malhotra_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/sidharth-malhotra-songs/na,EAoXKSjk_"
					},
					{
						"id": "467129",
						"name": "Neha Sharma",
						"role": "starring",
						"image": "https://c.saavncdn.com/573/Aaja-Tujhe-Pyar-Karu-Hindi-2107-20171026094123-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/neha-sharma-songs/GTTcXpQ5vNE_"
					},
					{
						"id": "2029028",
						"name": "Nilesh Ahuja",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/nilesh-ahuja-songs/GrTspM8TKSs_"
					},
					{
						"id": "455665",
						"name": "Kumaar",
						"role": "starring",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kumaar-songs/jXhf,IMIGGs_"
					}
				]
			},
			"releaseDate": "2021-02-12",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "qn50ItUC",
			"name": "Om Namah Shivay",
			"subtitle": "Shankar Mahadevan, Kedar Pandit - My Devotional Favourites - Shankar Mahadevan",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/om-namah-shivay/AQZeAT1EYnA",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/769/My-Devotional-Favourites-Shankar-Mahadevan-Hindi-2014-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/769/My-Devotional-Favourites-Shankar-Mahadevan-Hindi-2014-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/769/My-Devotional-Favourites-Shankar-Mahadevan-Hindi-2014-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2014,
			"playCount": 8733095,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Kedar Pandit",
			"song": null,
			"albumId": "1452339",
			"album": "My Devotional Favourites - Shankar Mahadevan",
			"label": "Strumm Entertainment",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy3U14B77AbcX6X5mxn5BmRtGR7EWoO6t/znrvExV6HmvaPEEW7rr5MBw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/my-devotional-favourites-shankar-mahadevan/GdgayVybuAw_",
			"duration": 509,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "Om Namah Shivay-Lyrics",
			"starred": false,
			"copyrightText": "©  2014 Strumm Entertainment",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "455275",
						"name": "Shankar Mahadevan",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Shankar_Mahadevan_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/shankar-mahadevan-songs/,QE2Oks7qzI_"
					},
					{
						"id": "456235",
						"name": "Kedar Pandit",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/068/Blessings-From-My-God-Shiva-Hindi-2011-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kedar-pandit-songs/3,xXBy4ua,I_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "456235",
						"name": "Kedar Pandit",
						"role": "music",
						"image": "https://c.saavncdn.com/068/Blessings-From-My-God-Shiva-Hindi-2011-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/kedar-pandit-songs/3,xXBy4ua,I_"
					},
					{
						"id": "455275",
						"name": "Shankar Mahadevan",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Shankar_Mahadevan_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/shankar-mahadevan-songs/,QE2Oks7qzI_"
					}
				]
			},
			"releaseDate": "2014-02-01",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "i4iDGcj6",
			"name": "Meri Maa Ke Barabar Koi Nahi",
			"subtitle": "Jubin Nautiyal, Payal Dev - Meri Maa Ke Barabar Koi Nahi",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/meri-maa-ke-barabar-koi-nahi/GVwCdTNTXQU",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/832/Meri-Maa-Ke-Barabar-Koi-Nahi-Hindi-2021-20220307121001-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/832/Meri-Maa-Ke-Barabar-Koi-Nahi-Hindi-2021-20220307121001-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/832/Meri-Maa-Ke-Barabar-Koi-Nahi-Hindi-2021-20220307121001-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2021,
			"playCount": 83305338,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Payal Dev",
			"song": null,
			"albumId": "30066271",
			"album": "Meri Maa Ke Barabar Koi Nahi",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy62R4jT6CU1tjtP/RExPzBiMRoVHUxrYpKdshP2R0ZpkX/ErNfwYLBhw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/meri-maa-ke-barabar-koi-nahi/EIICtTXEp4k_",
			"duration": 298,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "meri maan ke baraabar koyi nahin",
			"starred": false,
			"copyrightText": "℗ 2021 Super Cassettes Industries Private Limited",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "881158",
						"name": "Jubin Nautiyal",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
					},
					{
						"id": "653208",
						"name": "Payal Dev",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Payal_Dev_001_20191220112849_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/payal-dev-songs/szd0Qcak34Q_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "653208",
						"name": "Payal Dev",
						"role": "music",
						"image": "https://c.saavncdn.com/artists/Payal_Dev_001_20191220112849_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/payal-dev-songs/szd0Qcak34Q_"
					},
					{
						"id": "881158",
						"name": "Jubin Nautiyal",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
					},
					{
						"id": "653208",
						"name": "Payal Dev",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Payal_Dev_001_20191220112849_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/payal-dev-songs/szd0Qcak34Q_"
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
			"releaseDate": "2021-10-05",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "eC-s1N7u",
			"name": "Om Namah Shivay",
			"subtitle": "Navin Tripathi, Gul Saxena, Navin-Manish - Om Namah Shivay",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/om-namah-shivay/FStGQkV,AEY",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/341/Om-Namah-Shivay-Hindi-2022-20211203090853-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/341/Om-Namah-Shivay-Hindi-2022-20211203090853-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/341/Om-Namah-Shivay-Hindi-2022-20211203090853-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2022,
			"playCount": 6988474,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Navin Tripathi, Gul Saxena, Navin-Manish, Manish Tripathi",
			"song": null,
			"albumId": "31258028",
			"album": "Om Namah Shivay",
			"label": "Namah by Koinage",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDytZrql+JB+nu7ChVWK6mNvmuJMzwcc6sRIAXGLCoZi5YlAL685WTfxhw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/om-namah-shivay/byu8v6dYmU4_",
			"duration": 186,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "ॐ नमः शिवाय, ॐ नमः शिवाय",
			"starred": false,
			"copyrightText": "℗ 2021 Koinage Records",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "9234147",
						"name": "Navin Tripathi",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Navin_Tripathi_000_20230530122911_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-tripathi-songs/I8d6AARybOg_"
					},
					{
						"id": "5278908",
						"name": "Gul Saxena",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/636/Radhe-Radhe-Barsane-Wali-Radhe-Zee-Music-Devotional-Hindi-2019-20190410115146-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gul-saxena-songs/T4JFXVEbaqg_"
					},
					{
						"id": "841908",
						"name": "Navin-Manish",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/604/Kya-Ho-Gaya-Hindi-2019-20190114135034-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-manish-songs/MTai7A2,LCU_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "9234147",
						"name": "Navin Tripathi",
						"role": "music",
						"image": "https://c.saavncdn.com/artists/Navin_Tripathi_000_20230530122911_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-tripathi-songs/I8d6AARybOg_"
					},
					{
						"id": "5278908",
						"name": "Gul Saxena",
						"role": "music",
						"image": "https://c.saavncdn.com/636/Radhe-Radhe-Barsane-Wali-Radhe-Zee-Music-Devotional-Hindi-2019-20190410115146-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gul-saxena-songs/T4JFXVEbaqg_"
					},
					{
						"id": "841908",
						"name": "Navin-Manish",
						"role": "music",
						"image": "https://c.saavncdn.com/604/Kya-Ho-Gaya-Hindi-2019-20190114135034-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-manish-songs/MTai7A2,LCU_"
					},
					{
						"id": "5903886",
						"name": "Manish Tripathi",
						"role": "music",
						"image": "https://c.saavncdn.com/artists/Manish_Tripathi_000_20221122131600_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/manish-tripathi-songs/4wREX6toGZU_"
					},
					{
						"id": "9234147",
						"name": "Navin Tripathi",
						"role": "lyricist",
						"image": "https://c.saavncdn.com/artists/Navin_Tripathi_000_20230530122911_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/navin-tripathi-songs/I8d6AARybOg_"
					},
					{
						"id": "5903886",
						"name": "Manish Tripathi",
						"role": "lyricist",
						"image": "https://c.saavncdn.com/artists/Manish_Tripathi_000_20221122131600_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/manish-tripathi-songs/4wREX6toGZU_"
					}
				]
			},
			"releaseDate": "2022-01-03",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "Eb9_9BKF",
			"name": "Hanuman Chalisa",
			"subtitle": "Sachet Tandon - Hanuman Chalisa",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/hanuman-chalisa/NQpSbk1yfHU",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/980/Hanuman-Chalisa-Hindi-2020-20200720214001-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/980/Hanuman-Chalisa-Hindi-2020-20200720214001-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/980/Hanuman-Chalisa-Hindi-2020-20200720214001-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2020,
			"playCount": 48482469,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Bharat Goel",
			"song": null,
			"albumId": "21557592",
			"album": "Hanuman Chalisa",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyNX883DnV6/PNr4sqjyySVG/fSQeRCozPgcxrfeJfs53GY+cxTRUH4xw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/hanuman-chalisa/,QzwY3KWbPo_",
			"duration": 320,
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
			"copyrightText": "℗ 2020 Super Cassettes Industries Private Limited",
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
						"id": "702453",
						"name": "Bharat Goel",
						"role": "music",
						"image": "https://c.saavncdn.com/808/Sarcastic-Saiyaan-Hindi-2019-20190402195858-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/bharat-goel-songs/OebeATJODdQ_"
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
						"id": "455090",
						"name": "Traditional",
						"role": "lyricist",
						"image": "https://c.saavncdn.com/186/Krishna-Bhajans-Hindi-2010-150x150_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/traditional-songs/1W8p9iMeXm4_"
					}
				]
			},
			"releaseDate": "2020-07-21",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "e3RSZakk",
			"name": "Radha Krishna Serial All Song",
			"subtitle": "Gaurav Pareek, Narayan Pareek, Gaurav Pandiya, Lakshminarayan Pareek, Komal Pareek, Laxmi Narayan Pareek - Radha Krishna Serial All Song",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/radha-krishna-serial-all-song/FVs5Yi5RXFg",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/872/Radha-Krishna-Serial-All-Song-Hindi-2022-20220218171711-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/872/Radha-Krishna-Serial-All-Song-Hindi-2022-20220218171711-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/872/Radha-Krishna-Serial-All-Song-Hindi-2022-20220218171711-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2022,
			"playCount": 2068061,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Gaurav Pareek",
			"song": null,
			"albumId": "33019801",
			"album": "Radha Krishna Serial All Song",
			"label": "Gaurav Pareek Studio",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy+wBxuDKcgTSlwhxVzVri+CkwY5i2tQLsIAFyiMoUiGlli4v7OFZYlhw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/radha-krishna-serial-all-song/J,CmHE1AZvQ_",
			"duration": 756,
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
			"copyrightText": "© 2022 Gaurav Pareek",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "8858542",
						"name": "Gaurav Pareek",
						"role": "primary_artists",
						"image": "http://c.saavncdn.com/artists/Gaurav_Pareek_000_20210121122551_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gaurav-pareek-songs/NAV9iZM-7F8_"
					},
					{
						"id": "9092291",
						"name": "Narayan Pareek",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/narayan-pareek-songs/KDXq04HDrYs_"
					},
					{
						"id": "9241993",
						"name": "Gaurav Pandiya",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gaurav-pandiya-songs/7BVZbJ4ztvs_"
					},
					{
						"id": "12486061",
						"name": "Lakshminarayan Pareek",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/lakshminarayan-pareek-songs/FMIg7ainYiA_"
					},
					{
						"id": "9142839",
						"name": "Komal Pareek",
						"role": "primary_artists",
						"image": "http://c.saavncdn.com/artists/Komal_Pareek_000_20210122082030_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/komal-pareek-songs/CPbA4uVPKDc_"
					},
					{
						"id": "9146659",
						"name": "Laxmi Narayan Pareek",
						"role": "primary_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/laxmi-narayan-pareek-songs/bBK,NFHFusg_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "8858542",
						"name": "Gaurav Pareek",
						"role": "music",
						"image": "http://c.saavncdn.com/artists/Gaurav_Pareek_000_20210121122551_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gaurav-pareek-songs/NAV9iZM-7F8_"
					},
					{
						"id": "8858542",
						"name": "Gaurav Pareek",
						"role": "singer",
						"image": "http://c.saavncdn.com/artists/Gaurav_Pareek_000_20210121122551_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gaurav-pareek-songs/NAV9iZM-7F8_"
					},
					{
						"id": "9092291",
						"name": "Narayan Pareek",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/narayan-pareek-songs/KDXq04HDrYs_"
					},
					{
						"id": "9241993",
						"name": "Gaurav Pandiya",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/gaurav-pandiya-songs/7BVZbJ4ztvs_"
					},
					{
						"id": "12486061",
						"name": "Lakshminarayan Pareek",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/lakshminarayan-pareek-songs/FMIg7ainYiA_"
					},
					{
						"id": "9142839",
						"name": "Komal Pareek",
						"role": "singer",
						"image": "http://c.saavncdn.com/artists/Komal_Pareek_000_20210122082030_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/komal-pareek-songs/CPbA4uVPKDc_"
					},
					{
						"id": "9146659",
						"name": "Laxmi Narayan Pareek",
						"role": "singer",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/laxmi-narayan-pareek-songs/bBK,NFHFusg_"
					}
				]
			},
			"releaseDate": "2022-02-23",
			"trillerAvailable": false,
			"lyricsId": null
		},
		{
			"id": "cGByA4ik",
			"name": "Shiv Tandav",
			"subtitle": "Ananya Basu - Shiv Tandav",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/shiv-tandav/Ey8pSDUEXlg",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/192/Shiv-Tandav-Hindi-2018-20180831-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/192/Shiv-Tandav-Hindi-2018-20180831-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/192/Shiv-Tandav-Hindi-2018-20180831-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2018,
			"playCount": 13735576,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "D. Sushant",
			"song": null,
			"albumId": "13762534",
			"album": "Shiv Tandav",
			"label": "",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyQgi26X9WswaFyT8Sefe4KqYM78ZiDyTZiaSrEALZL6/srcVRWTHvvRw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/shiv-tandav/WoQ8Ydpt3sA_",
			"duration": 261,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "vimohana hi dehana tu shankarasya chintnam",
			"starred": false,
			"copyrightText": "Super Cassettes Industries Private Limited",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "3896775",
						"name": "Ananya Basu",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Ananya_Basu_20190927133103_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/ananya-basu-songs/fWo1Oo2M24o_"
					}
				],
				"featuredArtists": [],
				"artists": [
					{
						"id": "824894",
						"name": "D. Sushant",
						"role": "music",
						"image": "https://c.saavncdn.com/660/Jai-Ho-Bankey-Bihari-Hindi-2015-150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/d.-sushant-songs/ThxCzc9YOuI_"
					},
					{
						"id": "3896775",
						"name": "Ananya Basu",
						"role": "singer",
						"image": "https://c.saavncdn.com/artists/Ananya_Basu_20190927133103_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/ananya-basu-songs/fWo1Oo2M24o_"
					}
				]
			},
			"releaseDate": "2018-08-31",
			"trillerAvailable": false,
			"lyricsId": ""
		},
		{
			"id": "gjS5DmKi",
			"name": "Har Har Shambhu Shiv Mahadeva",
			"subtitle": "Jeetu Sharma ft. Abhilipsa Panda - Har Har Shambhu Shiv Mahadeva",
			"type": "song",
			"url": "https://www.jiosaavn.com/song/har-har-shambhu-shiv-mahadeva/FwI4BDBdfFo",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/224/Har-Har-Shambhu-Shiv-Mahadeva-Hindi-2022-20220729065345-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/224/Har-Har-Shambhu-Shiv-Mahadeva-Hindi-2022-20220729065345-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/224/Har-Har-Shambhu-Shiv-Mahadeva-Hindi-2022-20220729065345-500x500.jpg"
				}
			],
			"language": "hindi",
			"year": 2022,
			"playCount": 54108876,
			"explicitContent": false,
			"listCount": 0,
			"listType": "",
			"list": "",
			"music": "Jeetu Sharma",
			"song": null,
			"albumId": "36866647",
			"album": "Har Har Shambhu Shiv Mahadeva",
			"label": "Jeetu Sharma",
			"origin": "none",
			"isDolbyContent": false,
			"320kbps": true,
			"downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDynIgZ1wmh89uem7a2rpRyuYZP7Fxgy+YbGZtLWjoKbApMzSALPO4YKxw7tS9a8Gtq",
			"albumUrl": "https://www.jiosaavn.com/album/har-har-shambhu-shiv-mahadeva/spB7,HnxMY8_",
			"duration": 359,
			"rights": {
				"code": "0",
				"cacheable": "true",
				"delete_cached_object": "false",
				"reason": ""
			},
			"cacheState": "false",
			"hasLyrics": true,
			"lyricsSnippet": "हर-हर शंभु (शंभु), शंभु (शंभु), शिव महादेवा",
			"starred": false,
			"copyrightText": "℗ 2022 Jeetu Sharma",
			"artistMap": {
				"primaryArtists": [
					{
						"id": "3124014",
						"name": "Jeetu Sharma",
						"role": "primary_artists",
						"image": "https://c.saavncdn.com/artists/Jeetu_Sharma_000_20230217091836_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jeetu-sharma-songs/LUvaO6vCPV4_"
					}
				],
				"featuredArtists": [
					{
						"id": "13033924",
						"name": "Abhilipsa Panda",
						"role": "featured_artists",
						"image": "",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/abhilipsa-panda-songs/MMkQNb0k5F8_"
					}
				],
				"artists": [
					{
						"id": "3124014",
						"name": "Jeetu Sharma",
						"role": "music",
						"image": "https://c.saavncdn.com/artists/Jeetu_Sharma_000_20230217091836_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jeetu-sharma-songs/LUvaO6vCPV4_"
					},
					{
						"id": "3124014",
						"name": "Jeetu Sharma",
						"role": "lyricist",
						"image": "https://c.saavncdn.com/artists/Jeetu_Sharma_000_20230217091836_150x150.jpg",
						"type": "artist",
						"url": "https://www.jiosaavn.com/artist/jeetu-sharma-songs/LUvaO6vCPV4_"
					}
				]
			},
			"releaseDate": "2022-05-02",
			"trillerAvailable": false,
			"lyricsId": ""
		}
	]
}
```

+++

|            Query Parameter            | Description |                Required                |
| :-----------------------------------: | :---------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] |   Song ID   | [!badge variant="primary" text="True"] |
