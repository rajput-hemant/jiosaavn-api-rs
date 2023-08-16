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
