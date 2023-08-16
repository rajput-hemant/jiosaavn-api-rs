---
order: 90
icon: stack
---

# Album Details

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Album details by album ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/album?id=25500145
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/album?id=25500145' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Album details fetched successfully!",
	"data": {
		"id": "25500145",
		"name": "Ram Siya Ram",
		"subtitle": "Sachet Tandon",
		"type": "album",
		"language": "hindi",
		"playCount": 0,
		"explicit": false,
		"year": 2021,
		"url": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
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
		"releaseDate": null,
		"artistMap": {
			"primaryArtists": [
				{
					"id": "3623110",
					"name": "Sachet Tandon",
					"role": "",
					"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
					"type": "artist",
					"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
				}
			],
			"featuredArtists": [],
			"artists": [
				{
					"id": "3623110",
					"name": "Sachet Tandon",
					"role": "",
					"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
					"type": "artist",
					"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
				}
			]
		},
		"songCount": 1,
		"copyrightText": "© 2021 Super Cassettes Industries Private Limited",
		"isDolbyContent": false,
		"songs": [
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
				"playCount": 61463043,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Poonam Thakkar",
				"song": null,
				"albumId": "25500145",
				"album": "Ram Siya Ram",
				"label": "",
				"origin": "album",
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
				"cacheState": "",
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
}
```

+++

|            Query Parameter            | Description |                Required                |
| :-----------------------------------: | :---------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] |  Album ID   | [!badge variant="primary" text="True"] |

## Album details by link

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/album?link=https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/album?link=https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Album details fetched successfully!",
	"data": {
		"id": "25500145",
		"name": "Ram Siya Ram",
		"subtitle": "Sachet Tandon",
		"type": "album",
		"language": "hindi",
		"playCount": 0,
		"explicit": false,
		"year": 2021,
		"url": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
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
		"releaseDate": null,
		"artistMap": {
			"primaryArtists": [
				{
					"id": "3623110",
					"name": "Sachet Tandon",
					"role": "",
					"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
					"type": "artist",
					"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
				}
			],
			"featuredArtists": [],
			"artists": [
				{
					"id": "3623110",
					"name": "Sachet Tandon",
					"role": "",
					"image": "https://c.saavncdn.com/artists/Sachet_Tandon_001_20191130070910_150x150.jpg",
					"type": "artist",
					"url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
				}
			]
		},
		"songCount": 1,
		"copyrightText": "© 2021 Super Cassettes Industries Private Limited",
		"isDolbyContent": false,
		"songs": [
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
				"playCount": 61463043,
				"explicitContent": false,
				"listCount": 0,
				"listType": "",
				"list": "",
				"music": "Poonam Thakkar",
				"song": null,
				"albumId": "25500145",
				"album": "Ram Siya Ram",
				"label": "",
				"origin": "album",
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
				"cacheState": "",
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
}
```

+++

|             Query Parameter             |              Description               |                Required                |
| :-------------------------------------: | :------------------------------------: | :------------------------------------: |
| [!badge variant="contrast" text="link"] | Album link from <https://jiosaavn.com> | [!badge variant="primary" text="True"] |
