---
order: 10
icon: rss
---

# Radio

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Create a radio station

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/radio/create?name=arijit+singh&type=artist&language=hindi
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/radio/create?name=arijit+singh&type=artist&language=hindi' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Radio created successfully",
  "data": {
    "stationid": "554BiiZWBhKTnGFN8gtibAo-jpD3W13NJhephHpVFm,Pt2PDx7fzVA__~^~artist_radio~^~459320"
  }
}
```

+++

|             **Query Parameter**             |                                                                                                    **Description**                                                                                                     |              **Required**               |
| :-----------------------------------------: | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :-------------------------------------: |
|   [!badge variant="contrast" text="name"]   |                                                                                                          Name                                                                                                          | [!badge variant="primary" text="True"]  |
|   [!badge variant="contrast" text="type"]   |                                                                                   Radio Station Type `Artist`, `Featured`, `Entity`                                                                                    | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="language"] | one or more comma separated languages </br>`hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,`gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`, `rajasthani`, `odia`, `assamese` | [!badge variant="primary" text="False"] |

## Get radio station songs

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/radio/songs?id=554BiiZWBhKTnGFN8gtibAo-jpD3W13NJhephHpVFm,Pt2PDx7fzVA__~^~artist_radio~^~459320
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/radio/songs?id=554BiiZWBhKTnGFN8gtibAo-jpD3W13NJhephHpVFm,Pt2PDx7fzVA__~^~artist_radio~^~459320' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Radio songs fetched successfully",
  "data": {
    "stationid": "554BiiZWBhKTnGFN8gtibAo-jpD3W13NJhephHpVFm,Pt2PDx7fzVA__",
    "songs": [
      {
        "id": "B0kVSMrC",
        "name": "Bandeya (From &quot;Dil Juunglee&quot;)",
        "subtitle": "Toshi Sabri ft. Arijit Singh - Bandeya (From &quot;Dil Juunglee&quot;)",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/bandeya-from-dil-juunglee/MlgAZyd9RXA",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/267/Bandeya-From-Dil-Juunglee--Hindi-2018-20180222044014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/267/Bandeya-From-Dil-Juunglee--Hindi-2018-20180222044014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/267/Bandeya-From-Dil-Juunglee--Hindi-2018-20180222044014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2018,
        "playCount": 48656489,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Sharib Sabri, Toshi Sabri",
        "song": null,
        "albumId": "12293787",
        "album": "Bandeya (From &quot;Dil Juunglee&quot;)",
        "label": "Pooja Music",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy5kMxmDt9sDccDV+I7AFqlmAjuE+zGYA0jOPfIvLD7hpRZh0pb9ruHxw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/bandeya-from-dil-juunglee/4d0KjtFv6c0_",
        "duration": 185,
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
        "copyrightText": "(P) 2018 Pooja Music / Under Exclusive Distribution to Sony Music Entertainment India Pvt. Ltd.",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "711885",
              "name": "Toshi Sabri",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Toshi_Sabri_20200122132449_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/toshi-sabri-songs/18rWRCchxWU_"
            }
          ],
          "featuredArtists": [
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "featured_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "artists": [
            {
              "id": "470943",
              "name": "Sharib Sabri",
              "role": "music",
              "image": "https://c.saavncdn.com/267/Bandeya-From-Dil-Juunglee--Hindi-2018-20180222044014-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-sabri-songs/pwAZa24diWs_"
            },
            {
              "id": "711885",
              "name": "Toshi Sabri",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Toshi_Sabri_20200122132449_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/toshi-sabri-songs/18rWRCchxWU_"
            },
            {
              "id": "711885",
              "name": "Toshi Sabri",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Toshi_Sabri_20200122132449_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/toshi-sabri-songs/18rWRCchxWU_"
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
              "id": "483977",
              "name": "Dr. Devendra Kafir",
              "role": "lyricist",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dr.-devendra-kafir-songs/wybAoVO9TvY_"
            },
            {
              "id": "675262",
              "name": "Taapsee Pannu",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Taapsee_Pannu_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/taapsee-pannu-songs/n3EBFVeZm2Q_"
            },
            {
              "id": "480242",
              "name": "Saqib Saleem",
              "role": "starring",
              "image": "https://c.saavncdn.com/540/Asathoma-Sadgamaya-Kannada-2018-20180402-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/saqib-saleem-songs/j,MthBpAeJA_"
            }
          ]
        },
        "releaseDate": "2018-02-22",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "mHdeoOP-",
        "name": "Kalank (Title Track)",
        "subtitle": "Pritam, Arijit Singh - Kalank",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/kalank-title-track/HSAPVBt-Zx4",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/784/Kalank-Hindi-2019-20200508163312-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/784/Kalank-Hindi-2019-20200508163312-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/784/Kalank-Hindi-2019-20200508163312-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2019,
        "playCount": 166230860,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "15394623",
        "album": "Kalank",
        "label": "Zee Music Co.",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy68grFT2K5SpXChT3NC630o2Xq54gqC18RT2a99z7HPTsZ4a7edc7sBw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/kalank/Jv-F,nN0JoY_",
        "duration": 311,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Kalank (Title Track)-Lyrics",
        "starred": false,
        "copyrightText": "© 2019 Zee Music Company",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
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
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Amitabh_Bhattacharya_000_20220916184017_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "669125",
              "name": "Aditya Roy Kapur",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Aditya_Roy_Kapur_20201027070801_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/aditya-roy-kapur-songs/2rYodGIZ25w_"
            },
            {
              "id": "455575",
              "name": "Sanjay Dutt",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sanjay_Dutt_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sanjay-dutt-songs/NJlT9amnRWw_"
            },
            {
              "id": "455113",
              "name": "Madhuri Dixit",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Madhuri_Dixit_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/madhuri-dixit-songs/he0RI4RDPKM_"
            },
            {
              "id": "467228",
              "name": "Sonakshi Sinha",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sonakshi_Sinha_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sonakshi-sinha-songs/kmJYu,,v05Q_"
            },
            {
              "id": "511658",
              "name": "Alia Bhatt",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Alia_Bhatt_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alia-bhatt-songs/,henzzVDXDQ_"
            },
            {
              "id": "511656",
              "name": "Varun Dhawan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Varun_Dhawan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/varun-dhawan-songs/nPLp3EdUdaA_"
            }
          ]
        },
        "releaseDate": "2019-04-14",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "OhnxgY6_",
        "name": "Khairiyat",
        "subtitle": "Pritam, Arijit Singh - Chhichhore",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/khairiyat/PwAFSRNpAWw",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/937/Chhichhore-Hindi-2019-20190904104023-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/937/Chhichhore-Hindi-2019-20190904104023-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/937/Chhichhore-Hindi-2019-20190904104023-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2019,
        "playCount": 170764911,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "17132913",
        "album": "Chhichhore",
        "label": "",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyYhe68adzBZAKJQK9qjdF4unN9Zgscia9R+nDZkn3d967D1djfXn5Vxw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/chhichhore/V4F3M5,cNb4_",
        "duration": 280,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Tumhare bin deewane ka kya haal hai",
        "starred": false,
        "copyrightText": "© 2019 Super Cassettes Industries Private Limited",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
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
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Amitabh_Bhattacharya_000_20220916184017_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "2180329",
              "name": "Sushant Singh Rajput",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sushant_Singh_Rajput_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sushant-singh-rajput-songs/5lthUoFU2hM_"
            },
            {
              "id": "477854",
              "name": "Shraddha Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shraddha_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shraddha-kapoor-songs/97qRLkdq3gI_"
            },
            {
              "id": "8190578",
              "name": "Sushant Singh Rajput",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sushant-singh-rajput-songs/hr0WpXzfSAY_"
            },
            {
              "id": "457932",
              "name": "Prateik Babbar",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Prateik_Babbar_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/prateik-babbar-songs/Wd7rExbbZeE_"
            },
            {
              "id": "477854",
              "name": "Shraddha Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shraddha_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shraddha-kapoor-songs/97qRLkdq3gI_"
            },
            {
              "id": "670950",
              "name": "Varun Sharma",
              "role": "starring",
              "image": "https://c.saavncdn.com/972/Warning-Hindi-2013-20190618094753-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/varun-sharma-songs/bkzmfZw-Srs_"
            },
            {
              "id": "457932",
              "name": "Prateik Babbar",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Prateik_Babbar_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/prateik-babbar-songs/Wd7rExbbZeE_"
            },
            {
              "id": "5860917",
              "name": "Tushar Pandey",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tushar-pandey-songs/2EZS4j6epnQ_"
            },
            {
              "id": "2370737",
              "name": "Tahir Raj Bhasin",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tahir-raj-bhasin-songs/enKUVF6TFhU_"
            },
            {
              "id": "6531701",
              "name": "Naveen Polishetty",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/naveen-polishetty-songs/57z4ZGPbQSE_"
            }
          ]
        },
        "releaseDate": "2019-08-31",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "Ni6noMmw",
        "name": "Agar Tum Saath Ho",
        "subtitle": "Alka Yagnik, Arijit Singh - Tamasha",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/agar-tum-saath-ho/PgFdXxt9WkQ",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/994/Tamasha-Hindi-2015-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/994/Tamasha-Hindi-2015-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/994/Tamasha-Hindi-2015-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2015,
        "playCount": 268278976,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "A.R. Rahman",
        "song": null,
        "albumId": "1514383",
        "album": "Tamasha",
        "label": "",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy0GF7amDM+kR6MPgL6n1NCvHZAAa0+qM2vRmrw+iF1mtXa+r1rfsRTRw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/tamasha/59eZwnaaVkU_",
        "duration": 341,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "meri taraf aathaa har gam fisal jae",
        "starred": false,
        "copyrightText": "©  2015 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456269",
              "name": "A.R. Rahman",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/AR_Rahman_002_20210120084455_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/a.r.-rahman-songs/HhFyPLvlKN0_"
            },
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
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
              "id": "456259",
              "name": "Irshad Kamil",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Irshad_Kamil_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
            },
            {
              "id": "459632",
              "name": "Ranbir Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Ranbir_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ranbir-kapoor-songs/nqAfNmNB5II_"
            },
            {
              "id": "457931",
              "name": "Deepika Padukone",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Deepika_Padukone_002_20220131100114_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/deepika-padukone-songs/36LfdYhYGvc_"
            },
            {
              "id": "474125",
              "name": "Javed Sheikh",
              "role": "starring",
              "image": "https://c.saavncdn.com/273/Kahin-Pyar-Na-Ho-Jaye-Urdu-2019-20190621161415-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/javed-sheikh-songs/N3LffTRKyP0_"
            },
            {
              "id": "464333",
              "name": "Piyush Mishra",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Piyush_Mishra_20200507093157_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/piyush-mishra-songs/fWTOjq4IFIo_"
            },
            {
              "id": "465025",
              "name": "Sushma Seth",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sushma-seth-songs/B29CbumPwzg_"
            }
          ]
        },
        "releaseDate": "2015-10-16",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "zekEI3N6",
        "name": "Ae Dil Hai Mushkil Title Track (From &quot;Ae Dil Hai Mushkil&quot;)",
        "subtitle": "Pritam, Arijit Singh - Ae Dil Hai Mushkil (Original Motion Picture Soundtrack)",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/ae-dil-hai-mushkil-title-track-from-ae-dil-hai-mushkil/Cg0AdD0DeQU",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/257/Ae-Dil-Hai-Mushkil-Hindi-2016-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/257/Ae-Dil-Hai-Mushkil-Hindi-2016-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/257/Ae-Dil-Hai-Mushkil-Hindi-2016-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2016,
        "playCount": 221417102,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "2597301",
        "album": "Ae Dil Hai Mushkil (Original Motion Picture Soundtrack)",
        "label": "Sony Music Entertainment India Pvt. Ltd.",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy6pXiLG7rXcp9KyuTvmf/GaofE9hrMuU6AxEA8vY4CFuiAp2UUOrMnhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/ae-dil-hai-mushkil-original-motion-picture-soundtrack/2ByTwH6xkWk_",
        "duration": 268,
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
        "copyrightText": "(P) 2016 Sony Music Entertainment India Pvt. Ltd.",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
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
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Amitabh_Bhattacharya_000_20220916184017_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "459632",
              "name": "Ranbir Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Ranbir_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ranbir-kapoor-songs/nqAfNmNB5II_"
            },
            {
              "id": "462795",
              "name": "Anushka Sharma",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Anushka_Sharma_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/anushka-sharma-songs/kAK8j9P3jU8_"
            },
            {
              "id": "458923",
              "name": "Aishwarya Rai Bachchan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Aishwarya_Rai_Bachchan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/aishwarya-rai-bachchan-songs/487Vde71dZc_"
            },
            {
              "id": "476389",
              "name": "Fawad Khan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Fawad_Khan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/fawad-khan-songs/uK-,CwnTSxw_"
            }
          ]
        },
        "releaseDate": "2016-10-26",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "1I2Ua0sr",
        "name": "Ghungroo",
        "subtitle": "Vishal &amp; Shekhar, Arijit Singh, Shilpa Rao, Kumaar - War",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/ghungroo/QSFZZBUAREE",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/881/War-Hindi-2019-20191001104931-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/881/War-Hindi-2019-20191001104931-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/881/War-Hindi-2019-20191001104931-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2019,
        "playCount": 197842516,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Vishal &amp; Shekhar",
        "song": null,
        "albumId": "17182829",
        "album": "War",
        "label": "YRF Music",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyFylbzASTFAkrsODgufv5+nBDSzo3w0OXU2w1fMUYHrAmqCxvMLFM1Bw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/war/xcfTKYPOtCY_",
        "duration": 302,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "के घुंगरू टूट गए, के घुंगरू टूट गए",
        "starred": false,
        "copyrightText": "© 2019 YRF Music",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Vishal-Shekhar_20191130071357_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Shilpa_Rao_002_20230321111415_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
            },
            {
              "id": "455665",
              "name": "Kumaar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kumaar-songs/jXhf,IMIGGs_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Vishal-Shekhar_20191130071357_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
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
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Shilpa_Rao_002_20230321111415_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
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
              "id": "457799",
              "name": "Hrithik Roshan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Hrithik_Roshan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/hrithik-roshan-songs/g-Z62I,nkeQ_"
            },
            {
              "id": "701751",
              "name": "Tiger Shroff",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Tiger_Shroff_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tiger-shroff-songs/9fIi6jl-5e0_"
            },
            {
              "id": "673052",
              "name": "Vaani Kapoor",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vaani-kapoor-songs/nHZkXVoIvZw_"
            }
          ]
        },
        "releaseDate": "2019-09-30",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "UMFwT9MZ",
        "name": "Baaton Ko Teri",
        "subtitle": "Arijit Singh, Himesh Reshammiya - All Is Well",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/baaton-ko-teri/JSUtRiAJemk",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/768/All-Is-Well-Hindi-2015-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/768/All-Is-Well-Hindi-2015-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/768/All-Is-Well-Hindi-2015-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2015,
        "playCount": 54474818,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Himesh Reshammiya",
        "song": null,
        "albumId": "1287008",
        "album": "All Is Well",
        "label": "",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy6GAQIqyqoKWcz2CPyrbo5dCWGrCmdyioanwkpvxnUXVEGc+jjjYM4xw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/all-is-well/LEpJKE2nEsc_",
        "duration": 280,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "hoke juda ham naa juda ho sake",
        "starred": false,
        "copyrightText": "©  2015 ",
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
              "id": "455132",
              "name": "Himesh Reshammiya",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Himesh_Reshammiya_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/himesh-reshammiya-songs/N,m6H0-rqiY_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "455132",
              "name": "Himesh Reshammiya",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Himesh_Reshammiya_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/himesh-reshammiya-songs/N,m6H0-rqiY_"
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
              "id": "461011",
              "name": "Shabbir Ahmed",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Shabbir_Ahmed_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shabbir-ahmed-songs/CUgZ-dg1Y9w_"
            },
            {
              "id": "461363",
              "name": "Abhishek Bachchan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Abhishek_Bachchan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/abhishek-bachchan-songs/g4jK4dRXJfM_"
            },
            {
              "id": "467522",
              "name": "Asin",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Asin_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/asin-songs/PYizoWCK6nU_"
            },
            {
              "id": "455081",
              "name": "Rishi Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Rishi_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/rishi-kapoor-songs/One1RBrrrfs_"
            },
            {
              "id": "463467",
              "name": "Supriya Pathak",
              "role": "starring",
              "image": "https://c.saavncdn.com/978/Fasle-Na-Rahe-From-Jai-Chhathi-Maa--Hindi-2018-20181117104007-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/supriya-pathak-songs/,Ul-hiemorA_"
            }
          ]
        },
        "releaseDate": "2015-07-06",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "uFdDHxLR",
        "name": "Kasam Se",
        "subtitle": "Shekhar Ravjiani, Arijit Singh, Priya Saraiya - Kasam Se",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/kasam-se/BS4PdTxIe2E",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2023,
        "playCount": 148276,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Shekhar Ravjiani, Arijit Singh, Priya Saraiya",
        "song": null,
        "albumId": "46611781",
        "album": "Kasam Se",
        "label": "Ggaruda Music Pvt. Ltd.",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDySqnfkyQAGnVzGKYYOrYuLNwbknAOjPsYvJQ9jc5Q4E6EkKF9I4YKxhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/kasam-se/DHpUyVpRvkk_",
        "duration": 213,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Meri Khwahishon ka Sher",
        "starred": false,
        "copyrightText": "(P) 2023 Ggaruda Music Pvt. Ltd",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Shekhar_Ravjiani_002_20230801171429_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "673160",
              "name": "Priya Saraiya",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Priya_Saraiya_003_20211123104635_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/priya-saraiya-songs/DhHUFwiOfxE_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Shekhar_Ravjiani_002_20230801171429_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "673160",
              "name": "Priya Saraiya",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Priya_Saraiya_003_20211123104635_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/priya-saraiya-songs/DhHUFwiOfxE_"
            },
            {
              "id": "673160",
              "name": "Priya Saraiya",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Priya_Saraiya_003_20211123104635_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/priya-saraiya-songs/DhHUFwiOfxE_"
            }
          ]
        },
        "releaseDate": "2023-08-08",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "d-O4NgD6",
        "name": "Mast Magan",
        "subtitle": "Shankar-Ehsaan-Loy, Arijit Singh, Chinmayi Sripada - 2 States",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/mast-magan/FEUkBTpXcwU",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/930/2-States-2014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/930/2-States-2014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/930/2-States-2014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2014,
        "playCount": 85648385,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Shankar-Ehsaan-Loy",
        "song": null,
        "albumId": "1152075",
        "album": "2 States",
        "label": "",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyysavrqvO4VXTHqffNmCJ5g/jwe3d0Hhs2TVl/for+YQXHkJLTji2mhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/2-states/IZ-74lIYo14_",
        "duration": 280,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "man mast magan  man mast magan",
        "starred": false,
        "copyrightText": "©  2014 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455280",
              "name": "Shankar-Ehsaan-Loy",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Shankar-Ehsaan-Loy_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shankar-ehsaan-loy-songs/gaOQwwdJkNo_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "1984879",
              "name": "Chinmayi Sripada",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Chinmayi_Sripaada_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/chinmayi-sripada-songs/dUPdrrGYfPk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "455280",
              "name": "Shankar-Ehsaan-Loy",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Shankar-Ehsaan-Loy_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shankar-ehsaan-loy-songs/gaOQwwdJkNo_"
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
              "id": "1984879",
              "name": "Chinmayi Sripada",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Chinmayi_Sripaada_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/chinmayi-sripada-songs/dUPdrrGYfPk_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Amitabh_Bhattacharya_000_20220916184017_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "473371",
              "name": "Arjun Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Arjun_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arjun-kapoor-songs/xS,nRhjt3Tk_"
            },
            {
              "id": "511658",
              "name": "Alia Bhatt",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Alia_Bhatt_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alia-bhatt-songs/,henzzVDXDQ_"
            },
            {
              "id": "455634",
              "name": "Amrita Singh",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Amrita_Singh_20191127083227_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amrita-singh-songs/8V2RIx7NWW0_"
            },
            {
              "id": "455319",
              "name": "Ronit Roy",
              "role": "starring",
              "image": "https://c.saavncdn.com/808/Visva-Vina-Music-Of-The-Universe-2002-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ronit-roy-songs/9j,km7GWK8M_"
            }
          ]
        },
        "releaseDate": "2014-03-18",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "XBJ7e8fz",
        "name": "Dil Jhoom",
        "subtitle": "Mithoon, Arijit Singh - Gadar 2",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/dil-jhoom/KCohBhEIUUk",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2023,
        "playCount": 502491,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon, Arijit Singh",
        "song": null,
        "albumId": "47279722",
        "album": "Gadar 2",
        "label": "Zee Music Company",
        "origin": "radio",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy/CNMbKzm5JMUBuBjM7co6an84kbcF6U/M1SxNPDOHojo69W6VKZsZhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/gadar-2/zKU0zGJl3GY_",
        "duration": 304,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Khuda Ab Banata Nahi Aise Chehre",
        "starred": false,
        "copyrightText": "© 2023 Zee Music Company",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "669007",
              "name": "Sayeed Quadri",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Sayeed_Quadri_001_20210616180918_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sayeed-quadri-songs/CkMFUTAeb,c_"
            },
            {
              "id": "455627",
              "name": "Sunny Deol",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sunny_Deol_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sunny-deol-songs/OpniZFFn41o_"
            },
            {
              "id": "455082",
              "name": "Ameesha Patel",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Ameesha_Patel_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ameesha-patel-songs/CW4hmC7S0yo_"
            },
            {
              "id": "3972805",
              "name": "Utkarsh Sharma",
              "role": "starring",
              "image": "https://c.saavncdn.com/252/Aye-Mere-Pyare-Watan-English-2018-20180816171648-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/utkarsh-sharma-songs/EB8WERZKa9I_"
            }
          ]
        },
        "releaseDate": "2023-08-05",
        "trillerAvailable": false,
        "lyricsId": null
      }
    ]
  }
}
```

+++

|           **Query Parameter**            | **Description** |              **Required**               |
| :--------------------------------------: | :-------------: | :-------------------------------------: |
|  [!badge variant="contrast" text="id"]   |   Station ID    | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="count"] | Count of songs  | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="next"]  |   Next Index    | [!badge variant="primary" text="False"] |
