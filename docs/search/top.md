---
order: 800
icon: flame
---

# Search Top

Get top searches on JioSaavn

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/top
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/top' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Top searches fetched successfully!",
	"data": [
		{
			"id": "16242394",
			"name": "Let Me Down Slowly",
			"subtitle": "album",
			"type": "album",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/681/Let-Me-Down-Slowly-English-2018-20190607042218-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/681/Let-Me-Down-Slowly-English-2018-20190607042218-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/681/Let-Me-Down-Slowly-English-2018-20190607042218-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/album/let-me-down-slowly/jRvpUK4bADA_",
			"explicit": false
		},
		{
			"id": "878751",
			"name": "Paradox",
			"subtitle": "artist",
			"type": "artist",
			"image": "https://c.saavncdn.com/artists/Paradox_000_20230315143733.jpg",
			"url": "https://www.jiosaavn.com/artist/paradox-songs/nt-KmuU42uc_",
			"explicit": false
		},
		{
			"id": "42209980",
			"name": "9.45 P.m",
			"subtitle": "album",
			"type": "album",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/album/9.45-p.m/UrEH3dnwao0_",
			"explicit": false
		},
		{
			"id": "37625834",
			"name": "Calm Down",
			"subtitle": "album",
			"type": "album",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/596/Calm-Down-English-2022-20220826054039-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/596/Calm-Down-English-2022-20220826054039-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/596/Calm-Down-English-2022-20220826054039-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/album/calm-down/vPrtkFOXkzk_",
			"explicit": false
		},
		{
			"id": "846391",
			"name": "Love Songs",
			"subtitle": "artist",
			"type": "artist",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/143/Sensual-Touch-of-Love-Romantic-Music-Sensual-Vibes-for-Lovers-Romantic-Music-Massage-for-Two-Sex-First-Love-Romantic-Date-Candlelight-English-2016-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/143/Sensual-Touch-of-Love-Romantic-Music-Sensual-Vibes-for-Lovers-Romantic-Music-Massage-for-Two-Sex-First-Love-Romantic-Date-Candlelight-English-2016-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/143/Sensual-Touch-of-Love-Romantic-Music-Sensual-Vibes-for-Lovers-Romantic-Music-Massage-for-Two-Sex-First-Love-Romantic-Date-Candlelight-English-2016-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/artist/love-songs-songs/JvBHL1LUWoc_",
			"explicit": false
		},
		{
			"id": "6o8JoQ8b",
			"name": "Perfect",
			"subtitle": "song",
			"type": "song",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/286/WMG_190295851286-English-2017-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/286/WMG_190295851286-English-2017-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/286/WMG_190295851286-English-2017-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/song/perfect/RgdTexthD1E",
			"explicit": false
		},
		{
			"id": "HAk1__h6",
			"name": "Gangsta's Paradise ",
			"subtitle": "song",
			"type": "song",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/365/Gangsta-s-Paradise-English-1995-20190607041555-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/365/Gangsta-s-Paradise-English-1995-20190607041555-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/365/Gangsta-s-Paradise-English-1995-20190607041555-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/song/gangstas-paradise-/OCkAACtvXwU",
			"explicit": false
		},
		{
			"id": "40589480",
			"name": "People",
			"subtitle": "album",
			"type": "album",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/607/People-English-2022-20221207081653-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/607/People-English-2022-20221207081653-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/607/People-English-2022-20221207081653-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/album/people/UUUsva-VOPc_",
			"explicit": false
		},
		{
			"id": "Paem2Kf1",
			"name": "Starboy",
			"subtitle": "song",
			"type": "song",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/372/Starboy-English-2016-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/372/Starboy-English-2016-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/372/Starboy-English-2016-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/song/starboy/IAkOXEZ7UQI",
			"explicit": false
		},
		{
			"id": "1759517",
			"name": "Unstoppable",
			"subtitle": "album",
			"type": "album",
			"image": [
				{
					"quality": "50x50",
					"link": "https://c.saavncdn.com/552/Unstoppable-English-2016-50x50.jpg"
				},
				{
					"quality": "150x150",
					"link": "https://c.saavncdn.com/552/Unstoppable-English-2016-150x150.jpg"
				},
				{
					"quality": "500x500",
					"link": "https://c.saavncdn.com/552/Unstoppable-English-2016-500x500.jpg"
				}
			],
			"url": "https://www.jiosaavn.com/album/unstoppable/1Tm2aGl16CM_",
			"explicit": false
		}
	]
}
```

+++
