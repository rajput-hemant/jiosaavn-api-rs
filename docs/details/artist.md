---
order: 70
icon: person
---

# Artist Details

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Artist details by artist ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/artist?id=459320
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/artist?id=459320' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Artist details fetched successfully!",
  "data": {
    "id": "459320",
    "name": "Arijit Singh",
    "subtitle": "Artist • 20716607 Listeners",
    "image": [
      {
        "quality": "50x50",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_50x50.jpg"
      },
      {
        "quality": "150x150",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg"
      },
      {
        "quality": "500x500",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_500x500.jpg"
      }
    ],
    "followerCount": 14584087,
    "type": "artist",
    "isVerified": true,
    "dominantLanguage": "hindi",
    "dominantType": "singer",
    "topSongs": [
      {
        "id": "qZtKBMZ_",
        "name": "Apna Bana Le",
        "subtitle": "Arijit Singh, Sachin-Jigar - Bhediya",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/apna-bana-le/ATIfejZ9bWw",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2022,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Sachin-Jigar",
        "song": null,
        "albumId": "38682222",
        "album": "Bhediya",
        "label": "Zee Music Co.",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDylA8lQwDowJF0rwulZQb41oGFWui//Ii/9d8Rp0MdqrAGAnyTSi1MVxw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/bhediya/wSM2AOubajk_",
        "duration": 261,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Jag Ki Hirasat Se",
        "starred": false,
        "copyrightText": "© 2023 Zee Music Company",
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
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
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
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
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
              "id": "511656",
              "name": "Varun Dhawan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Varun_Dhawan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/varun-dhawan-songs/nPLp3EdUdaA_"
            },
            {
              "id": "701752",
              "name": "Kriti Sanon",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Kriti_Sanon_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kriti-sanon-songs/9UtnOColzrU_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "_rJmbKSP",
        "name": "Shayad",
        "subtitle": "Pritam, Arijit Singh - Love Aaj Kal (Original Motion Picture Soundtrack)",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/shayad/LxohXBZ7ZGM",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2020,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "19072107",
        "album": "Love Aaj Kal (Original Motion Picture Soundtrack)",
        "label": "Sony Music Entertainment India Pvt. Ltd.",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyT19PzAX7YyzgkHYWQybH7xN4MJMIg+vv6YsG3yMFglt8bvlJKKDQQRw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/love-aaj-kal-original-motion-picture-soundtrack/08dQgBZGh20_",
        "duration": 247,
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
        "copyrightText": "(P) 2020 Sony Music Entertainment India Pvt. Ltd.",
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
              "id": "456259",
              "name": "Irshad Kamil",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Irshad_Kamil_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
            },
            {
              "id": "5607405",
              "name": "Sara Ali Khan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sara_Ali_Khan_20200303113945_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sara-ali-khan-songs/R8Muu,CMC78_"
            },
            {
              "id": "3392054",
              "name": "Kartik Aaryan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Kartik_Aaryan_000_20211119092218_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kartik-aaryan-songs/s6R4Iv6pc3s_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "gTOlL2wh",
        "name": "Sawan Aaya Hai",
        "subtitle": "Tony Kakkar, Mithoon, Arijit Singh - Creature 3D",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/sawan-aaya-hai/FzwkXTgCQFs",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2014,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Tony Kakkar, Mithoon",
        "song": null,
        "albumId": "1194579",
        "album": "Creature 3D",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyev1qlIfUK4igRCF4+gjCLkq7zU4uI/aYhVaTB/dNYoVHm+0ney/T5xw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/creature-3d/TDDY3rBIUiw_",
        "duration": 288,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Sawan Aaya Hai-Lyrics",
        "starred": false,
        "copyrightText": "©  2014 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
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
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
            {
              "id": "456317",
              "name": "Bipasha Basu",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Bipasha_Basu_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/bipasha-basu-songs/CxaBGYXiR00_"
            },
            {
              "id": "740160",
              "name": "Imran Abbas Naqvi",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/imran-abbas-naqvi-songs/vGzqu5QGfaM_"
            },
            {
              "id": "740161",
              "name": "Bikramjeet Singh",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/bikramjeet-singh-songs/r,TxXGw7xLM_"
            },
            {
              "id": "468113",
              "name": "Deepraj Rana",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/deepraj-rana-songs/0cjTGgkRDAg_"
            },
            {
              "id": "466125",
              "name": "Mukul Dev",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mukul-dev-songs/n,RRVe0ndHs_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "2aaBp3ZE",
        "name": "Tumse Bhi Zyada",
        "subtitle": "Arijit Singh - Tadap",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/tumse-bhi-zyada/QgkKcwQDbXY",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2021,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "31262422",
        "album": "Tadap",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyGKfyjN+uwn9R72miHHSmmJ8iv1wv7QxfLFr1Qn7gNVsAe+/0pLQwdRw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/tadap/LiUPUh2JIdE_",
        "duration": 319,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "tumse bhee zyaada tumse pyaar kiya",
        "starred": false,
        "copyrightText": "℗ 2021 Super Cassettes Industries Private Limited",
        "artistMap": {
          "primaryArtists": [
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
              "id": "458223",
              "name": "Ashish Pandit",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/515/Love-Shagun-Hindi-2016-20191118095603-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ashish-pandit-songs/VhzkfC-tgFg_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "aRZbUYD7",
        "name": "Tum Hi Ho",
        "subtitle": "Arijit Singh - Aashiqui 2",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/tum-hi-ho/EToxUyFpcwQ",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2013,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon",
        "song": null,
        "albumId": "1139549",
        "album": "Aashiqui 2",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyRofh8YQPTqjR2tIxTYXqGEQHgqnNfsetouo4rsD0s4RZKObWAyHrEhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/aashiqui-2/-iNdCmFNV9o_",
        "duration": 262,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Tere Bina Kya Wajood Mera",
        "starred": false,
        "copyrightText": "©  2013 ",
        "artistMap": {
          "primaryArtists": [
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
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
              "id": "477854",
              "name": "Shraddha Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shraddha_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shraddha-kapoor-songs/97qRLkdq3gI_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "riNBfJ3P",
        "name": "Sanam Re",
        "subtitle": "Arijit Singh, Mithoon - Sanam Re",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/sanam-re/AgElcxJ6BGM",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2016,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon",
        "song": null,
        "albumId": "1699057",
        "album": "Sanam Re",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyrlecNtdN+6iCR2CebRPf8Tf/C/UqCdBhzkHfD38oIA4fPHKDwljT6hw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/sanam-re/usTMyKvgz4M_",
        "duration": 308,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "sanam re, sanam re, too mera sanam hua  re",
        "starred": false,
        "copyrightText": "©  2016 ",
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
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "531878",
              "name": "Pulkit Samrat",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pulkit-samrat-songs/XNeUtvztuY8_"
            },
            {
              "id": "505299",
              "name": "Yami Gautam",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Yami_Gautam_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/yami-gautam-songs/hK7MoHPut2M_"
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
              "id": "679274",
              "name": "Urvashi Rautela",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Urvashi_Rautela_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/urvashi-rautela-songs/GcTwpCvBIMs_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "lteSMrqT",
        "name": "Main Dhoondne Ko Zamaane Mein",
        "subtitle": "Gaurav Dagaonkar, Arijit Singh - Heartless",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/main-dhoondne-ko-zamaane-mein/HBwOYjlCRmc",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/112/Heartless-2014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/112/Heartless-2014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/112/Heartless-2014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2014,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Gaurav Dagaonkar",
        "song": null,
        "albumId": "1148662",
        "album": "Heartless",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDypARW5XI2blkziZ7sOnVcYvG5EN90llxl7glNPThVZzuw4CBAvu1A+Rw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/heartless/oSeU3sTMFgI_",
        "duration": 263,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "main dhoondhane ko zamaane mein jab vafaa nikalaa",
        "starred": false,
        "copyrightText": "©  2014 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "490085",
              "name": "Gaurav Dagaonkar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Gaurav_Dagaonkar_20191216113105_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gaurav-dagaonkar-songs/XPxdM,JR8tY_"
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
              "id": "490085",
              "name": "Gaurav Dagaonkar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Gaurav_Dagaonkar_20191216113105_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gaurav-dagaonkar-songs/XPxdM,JR8tY_"
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
              "id": "469700",
              "name": "Arafat Mehmood",
              "role": "lyricist",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arafat-mehmood-songs/7JfoXUgI2OM_"
            },
            {
              "id": "471684",
              "name": "Adhyayan Suman",
              "role": "starring",
              "image": "https://c.saavncdn.com/446/Soniyo-Version-2-0--Hindi-2019-20190430113919-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/adhyayan-suman-songs/ngbwwd6Sekc_"
            },
            {
              "id": "684210",
              "name": "Arianaayam",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arianaayam-songs/-nToxkJP6JA_"
            },
            {
              "id": "455529",
              "name": "Om Puri",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Om_Puri_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/om-puri-songs/Naa70mb2fgE_"
            },
            {
              "id": "461019",
              "name": "Deepti Naval",
              "role": "starring",
              "image": "https://c.saavncdn.com/000/default_Saregama_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/deepti-naval-songs/BtVBt-YTiJk_"
            },
            {
              "id": "461079",
              "name": "Shekhar Suman",
              "role": "starring",
              "image": "https://c.saavncdn.com/728/Shukriya-2004-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-suman-songs/PsCGUekUNCY_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "NCUGpKOW",
        "name": "Jhoome Jo Pathaan",
        "subtitle": "Vishal &amp; Shekhar, Arijit Singh, Sukriti Kakar, Vishal Dadlani, Shekhar Ravjiani, Kumaar - Pathaan",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/jhoome-jo-pathaan/Pis,dgR7eGQ",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2022,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Vishal &amp; Shekhar",
        "song": null,
        "albumId": "41039709",
        "album": "Pathaan",
        "label": "YRF Music",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy3hetKhG9ECxcSXbpIZnD+CJHn2cWmVEKctqtBPTgpb2MTK+56RDP6Bw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/pathaan/kw5AWtM1BZk_",
        "duration": 208,
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
        "copyrightText": "© 2022 YRF Music",
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
              "id": "670650",
              "name": "Sukriti Kakar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Sukriti_Kakar_002_20200314080054_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukriti-kakar-songs/fB0XSQjRbtM_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Vishal_Dadlani.png",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Shekhar_Ravjiani_002_20230801171429_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
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
              "id": "670650",
              "name": "Sukriti Kakar",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Sukriti_Kakar_002_20200314080054_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukriti-kakar-songs/fB0XSQjRbtM_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Vishal_Dadlani.png",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Shekhar_Ravjiani_002_20230801171429_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
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
              "id": "461361",
              "name": "Shah Rukh Khan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shah_Rukh_Khan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shah-rukh-khan-songs/tkXMVGTn-A0_"
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
              "id": "456054",
              "name": "John Abraham",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/John_Abraham_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/john-abraham-songs/62,5Pss67MY_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "IlbqfHKT",
        "name": "Phir Bhi Tumko Chaahunga",
        "subtitle": "Mithoon, Arijit Singh, Shashaa Tirupati - Half Girlfriend",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/phir-bhi-tumko-chaahunga/OQQJQBJ4fGc",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/441/Half-Girlfriend-Hindi-2017-20180622-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/441/Half-Girlfriend-Hindi-2017-20180622-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/441/Half-Girlfriend-Hindi-2017-20180622-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2017,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon",
        "song": null,
        "albumId": "10601320",
        "album": "Half Girlfriend",
        "label": "Zee Music Co.",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyQLZwquddgNe6Fz2oE4WCNyPJYVmH/v4r0KYodueIpQvipcg5i4jiARw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/half-girlfriend/Za3iBXuncjM_",
        "duration": 352,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "main phir bhee tumko chaahoongaa",
        "starred": false,
        "copyrightText": "2017 Zee Music Company",
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
            },
            {
              "id": "697946",
              "name": "Shashaa Tirupati",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Shashaa_Tirupati_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shashaa-tirupati-songs/,T0PZDWC4Xc_"
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
              "id": "702592",
              "name": "Mithoon",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
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
              "id": "697946",
              "name": "Shashaa Tirupati",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Shashaa_Tirupati_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shashaa-tirupati-songs/,T0PZDWC4Xc_"
            },
            {
              "id": "473441",
              "name": "Manoj Muntashir",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/259/Episode-1-Dekhte-Dekhte-From-Muntashir-Ki-Diary-Se--Hindi-2019-20190902135108-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/manoj-muntashir-songs/eaiDjU0BhyA_"
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
              "id": "477854",
              "name": "Shraddha Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shraddha_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shraddha-kapoor-songs/97qRLkdq3gI_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "Otl9gukS",
        "name": "Lambiyaan Si Judaiyaan",
        "subtitle": "Arijit Singh - Raabta",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/lambiyaan-si-judaiyaan/PxwHCBNFXGA",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/023/Raabta-Hindi-2017-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/023/Raabta-Hindi-2017-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/023/Raabta-Hindi-2017-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2017,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "JAM8",
        "song": null,
        "albumId": "10660301",
        "album": "Raabta",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyNm9/XAMnX2f7ACoqUKR0oj/Vd21Y4/6EaiW8jGeYVK8Os4qwlny6Rxw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/raabta/CKUSdZyuX0E_",
        "duration": 238,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "ਵੇ ਬੜੀ ਲੰਬੀਆਂ ਸੀ ਜੁਦਾਈਆਂ",
        "starred": false,
        "copyrightText": "©  2017 ",
        "artistMap": {
          "primaryArtists": [
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
              "id": "2663500",
              "name": "JAM8",
              "role": "music",
              "image": "https://c.saavncdn.com/023/Raabta-Hindi-2017-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jam8-songs/JAdIelXlNlQ_"
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
              "id": "701752",
              "name": "Kriti Sanon",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Kriti_Sanon_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kriti-sanon-songs/9UtnOColzrU_"
            },
            {
              "id": "3243424",
              "name": "Jim Sarbh",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jim-sarbh-songs/bFHCV3J-FY0_"
            },
            {
              "id": "670950",
              "name": "Varun Sharma",
              "role": "starring",
              "image": "https://c.saavncdn.com/972/Warning-Hindi-2013-20190618094753-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/varun-sharma-songs/bkzmfZw-Srs_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      }
    ],
    "topAlbums": [
      {
        "id": "47279722",
        "name": "Gadar 2",
        "subtitle": "Mithoon, Uttam Singh",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/gadar-2/zKU0zGJl3GY_",
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
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "461323",
              "name": "Uttam Singh",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/uttam-singh-songs/oNUhFwf,KLc_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "455127",
              "name": "Udit Narayan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_"
            },
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
            },
            {
              "id": "461323",
              "name": "Uttam Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/uttam-singh-songs/oNUhFwf,KLc_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "459733",
              "name": "Aditya Narayan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/aditya-narayan-songs/TY,T7F3OI64_"
            },
            {
              "id": "455636",
              "name": "Sukhwinder Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukhwinder-singh-songs/X04Pj7GRt40_"
            },
            {
              "id": "531639",
              "name": "Neeti Mohan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neeti-mohan-songs/3rVrdWgQlqs_"
            },
            {
              "id": "702452",
              "name": "Vishal Mishra",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-mishra-songs/f0sXoS0mUnE_"
            },
            {
              "id": "593966",
              "name": "Shehnaz Akhtar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shehnaz-akhtar-songs/2ToejJKEHDM_"
            },
            {
              "id": "684962",
              "name": "Sahil Akhtar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sahil-akhtar-songs/KPyKn19Z7i0_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            }
          ]
        },
        "songCount": 7,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "38682222",
        "name": "Bhediya",
        "subtitle": "Sachin-Jigar",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2022,
        "url": "https://www.jiosaavn.com/album/bhediya/wSM2AOubajk_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            },
            {
              "id": "640931",
              "name": "Sachin Sanghvi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-sanghvi-songs/2jDoqGtKfFE_"
            },
            {
              "id": "470069",
              "name": "Jigar Saraiya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jigar-saraiya-songs/1uFvGEnOrnI_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            },
            {
              "id": "4583855",
              "name": "Sakshi Holkar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sakshi-holkar-songs/wsANapvxb9Y_"
            },
            {
              "id": "482886",
              "name": "Siddharth Basrur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/siddharth-basrur-songs/uDwDFPrgbjI_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "455636",
              "name": "Sukhwinder Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukhwinder-singh-songs/X04Pj7GRt40_"
            },
            {
              "id": "3712926",
              "name": "Rashmeet Kaur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/rashmeet-kaur-songs/xQ7jDdbDGKw_"
            },
            {
              "id": "459583",
              "name": "Ash King",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ash-king-songs/V61FW0l-DEU_"
            },
            {
              "id": "473376",
              "name": "Divya Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/divya-kumar-songs/7oZnjNxjuR4_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ]
        },
        "songCount": 5,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "41039709",
        "name": "Pathaan",
        "subtitle": "Vishal &amp; Shekhar, Sanchit Balhara, Ankit Balhara",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2022,
        "url": "https://www.jiosaavn.com/album/pathaan/kw5AWtM1BZk_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
            },
            {
              "id": "672066",
              "name": "Sanchit Balhara",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sanchit-balhara-songs/34VS9v3X77U_"
            },
            {
              "id": "672069",
              "name": "Ankit Balhara",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-balhara-songs/AtNj5y6iSWI_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
            },
            {
              "id": "672066",
              "name": "Sanchit Balhara",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sanchit-balhara-songs/34VS9v3X77U_"
            },
            {
              "id": "672069",
              "name": "Ankit Balhara",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-balhara-songs/AtNj5y6iSWI_"
            },
            {
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
            },
            {
              "id": "461068",
              "name": "Caralisa Monteiro",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/caralisa-monteiro-songs/U4CwqcIkmsU_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "670650",
              "name": "Sukriti Kakar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukriti-kakar-songs/fB0XSQjRbtM_"
            },
            {
              "id": "4139728",
              "name": "Magdalena Supel",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/magdalena-supel-songs/ibMwOg5tQCo_"
            },
            {
              "id": "14921580",
              "name": "Maanya Narang",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/maanya-narang-songs/iMNTXvX1PWU_"
            },
            {
              "id": "7662618",
              "name": "Riya Duggal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/riya-duggal-songs/mmN5pW2L8fI_"
            }
          ]
        },
        "songCount": 4,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "43600311",
        "name": "Tu Jhoothi Main Makkaar",
        "subtitle": "Pritam",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/tu-jhoothi-main-makkaar/Zk6,htMIngg_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "745460",
              "name": "Nikhita Gandhi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/nikhita-gandhi-songs/PP1RAOLWPAM_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "460577",
              "name": "Charan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/charan-songs/1jooVizMjiI_"
            },
            {
              "id": "455129",
              "name": "Sunidhi Chauhan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-songs/ZIyMwJzHJwI_"
            },
            {
              "id": "909299",
              "name": "Shashwat Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shashwat-singh-songs/gJthH4uEIuI_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            },
            {
              "id": "455130",
              "name": "Shreya Ghoshal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
            },
            {
              "id": "473376",
              "name": "Divya Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/divya-kumar-songs/7oZnjNxjuR4_"
            }
          ]
        },
        "songCount": 6,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "30775211",
        "name": "Sooryavanshi",
        "subtitle": "Various Artists",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2021,
        "url": "https://www.jiosaavn.com/album/sooryavanshi/q3wQulyDlhg_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455494",
              "name": "Various Artists",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/various-artists-songs/ztKx8IUBme8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "3953689",
              "name": "Lijo George-Dj Chetas",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/lijo-george-dj-chetas-songs/OUMjvsOYLbw_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "1595701",
              "name": "Tanishk Bagchi",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanishk-bagchi-songs/Y-5m-phldpg_"
            },
            {
              "id": "458188",
              "name": "Kaushik",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kaushik-songs/uHkINkejJZs_"
            },
            {
              "id": "465738",
              "name": "Guddu",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/guddu-songs/C2MlvJiYwYc_"
            },
            {
              "id": "462319",
              "name": "Akash",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akash-songs/z5o8fN,xGHA_"
            },
            {
              "id": "658557",
              "name": "Pav Dharia",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pav-dharia-songs/FO8vGJq0VCQ_"
            },
            {
              "id": "457905",
              "name": "Viju Shah",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/viju-shah-songs/72XE57yXd5E_"
            },
            {
              "id": "1393898",
              "name": "Lijo George",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/lijo-george-songs/BLEppbJmz9E_"
            },
            {
              "id": "455652",
              "name": "Daler Mehndi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/daler-mehndi-songs/00FjmqGw0V8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "531639",
              "name": "Neeti Mohan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neeti-mohan-songs/3rVrdWgQlqs_"
            },
            {
              "id": "658557",
              "name": "Pav Dharia",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pav-dharia-songs/FO8vGJq0VCQ_"
            },
            {
              "id": "745460",
              "name": "Nikhita Gandhi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/nikhita-gandhi-songs/PP1RAOLWPAM_"
            },
            {
              "id": "455127",
              "name": "Udit Narayan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_"
            },
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
            }
          ]
        },
        "songCount": 5,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "16188900",
        "name": "Kabir Singh",
        "subtitle": "Various Artists",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2019,
        "url": "https://www.jiosaavn.com/album/kabir-singh/kLG-OKbVmvM_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455494",
              "name": "Various Artists",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/various-artists-songs/ztKx8IUBme8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "3623112",
              "name": "Sachet-Parampara",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachet-parampara-songs/-01nNmS1dCs_"
            },
            {
              "id": "702452",
              "name": "Vishal Mishra",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-mishra-songs/f0sXoS0mUnE_"
            },
            {
              "id": "481043",
              "name": "Akhil Sachdeva",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akhil-sachdeva-songs/wTX9Hfv1bmM_"
            },
            {
              "id": "743637",
              "name": "Amaal Mallik",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amaal-mallik-songs/hZrw5p6a5Sk_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "3623110",
              "name": "Sachet Tandon",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachet-tandon-songs/wVwhaAx3x6c_"
            },
            {
              "id": "702452",
              "name": "Vishal Mishra",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-mishra-songs/f0sXoS0mUnE_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "3623111",
              "name": "Parampara Tandon",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/parampara-tandon-songs/cJO7csRHiSM_"
            },
            {
              "id": "455137",
              "name": "Tulsi Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tulsi-kumar-songs/31Xl5D0eU5Y_"
            },
            {
              "id": "481043",
              "name": "Akhil Sachdeva",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akhil-sachdeva-songs/wTX9Hfv1bmM_"
            },
            {
              "id": "455130",
              "name": "Shreya Ghoshal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
            },
            {
              "id": "464656",
              "name": "Armaan Malik",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/armaan-malik-songs/1iZ7Gi0bi1Y_"
            }
          ]
        },
        "songCount": 9,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "46944081",
        "name": "Bawaal",
        "subtitle": "Mithoon, Akashdeep Sengupta, Tanishk Bagchi",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/bawaal/lr6LXftWpxc_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/016/Bawaal-Hindi-2023-20230722214515-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/016/Bawaal-Hindi-2023-20230722214515-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/016/Bawaal-Hindi-2023-20230722214515-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "3362043",
              "name": "Akashdeep Sengupta",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akashdeep-sengupta-songs/RDT4nQ,-jo8_"
            },
            {
              "id": "1595701",
              "name": "Tanishk Bagchi",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanishk-bagchi-songs/Y-5m-phldpg_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "3362043",
              "name": "Akashdeep Sengupta",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akashdeep-sengupta-songs/RDT4nQ,-jo8_"
            },
            {
              "id": "1595701",
              "name": "Tanishk Bagchi",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanishk-bagchi-songs/Y-5m-phldpg_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "3362043",
              "name": "Akashdeep Sengupta",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akashdeep-sengupta-songs/RDT4nQ,-jo8_"
            },
            {
              "id": "15171740",
              "name": "Laqshay Kapoor",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/laqshay-kapoor-songs/-IHL6kwwacQ_"
            },
            {
              "id": "461070",
              "name": "Kausar Munir",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kausar-munir-songs/ZIoQfneojLU_"
            },
            {
              "id": "1595701",
              "name": "Tanishk Bagchi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanishk-bagchi-songs/Y-5m-phldpg_"
            },
            {
              "id": "702452",
              "name": "Vishal Mishra",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-mishra-songs/f0sXoS0mUnE_"
            },
            {
              "id": "11770594",
              "name": "Zahrah S Khan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/zahrah-s-khan-songs/Hm0uVlKGSTI_"
            },
            {
              "id": "500223",
              "name": "Romy",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/romy-songs/Bhgw,VyE1OQ_"
            },
            {
              "id": "5290890",
              "name": "Shloke Lal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shloke-lal-songs/8WJq244rEQQ_"
            },
            {
              "id": "1611079",
              "name": "Pravesh Mallick",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pravesh-mallick-songs/VvhwG,UmYJw_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "473441",
              "name": "Manoj Muntashir",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/manoj-muntashir-songs/eaiDjU0BhyA_"
            }
          ]
        },
        "songCount": 4,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "1205636",
        "name": "Zid (Original Motion Picture Soundtrack)",
        "subtitle": "Sharib Toshi",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2014,
        "url": "https://www.jiosaavn.com/album/zid-original-motion-picture-soundtrack/xZZT9ec6oR0_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/722/Zid-Original-Motion-Picture-Soundtrack-Hindi-2014-20230331114652-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/722/Zid-Original-Motion-Picture-Soundtrack-Hindi-2014-20230331114652-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/722/Zid-Original-Motion-Picture-Soundtrack-Hindi-2014-20230331114652-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "3086629",
              "name": "Sharib Toshi",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-toshi-songs/H8srOKPS0sc_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "470943",
              "name": "Sharib Sabri",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-sabri-songs/pwAZa24diWs_"
            },
            {
              "id": "711885",
              "name": "Toshi Sabri",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/toshi-sabri-songs/18rWRCchxWU_"
            },
            {
              "id": "3086629",
              "name": "Sharib Toshi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-toshi-songs/H8srOKPS0sc_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "455129",
              "name": "Sunidhi Chauhan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-songs/ZIyMwJzHJwI_"
            },
            {
              "id": "470943",
              "name": "Sharib Sabri",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-sabri-songs/pwAZa24diWs_"
            },
            {
              "id": "760665",
              "name": "Sharib Toshi &amp; Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sharib-toshi-arijit-singh-songs/Wc7YM7EftI0_"
            },
            {
              "id": "4553988",
              "name": "Sunidhi Chauhan &amp; Sharib Sabri",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-sharib-sabri-songs/nqqQnHgXKhQ_"
            }
          ]
        },
        "songCount": 5,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "37079371",
        "name": "Laal Singh Chaddha",
        "subtitle": "Pritam",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2022,
        "url": "https://www.jiosaavn.com/album/laal-singh-chaddha/9YfgU0uh3KM_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/576/Laal-Singh-Chaddha-Hindi-2022-20220805121002-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/576/Laal-Singh-Chaddha-Hindi-2022-20220805121002-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/576/Laal-Singh-Chaddha-Hindi-2022-20220805121002-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "660771",
              "name": "Mohan Kannan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mohan-kannan-songs/PiFkLP84bHc_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "455125",
              "name": "Sonu Nigam",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sonu-nigam-songs/,kuQK6K6u0I_"
            },
            {
              "id": "14339182",
              "name": "Romy",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/romy-songs/qLuOaJ7lKRY_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "459325",
              "name": "Shadab Faridi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shadab-faridi-songs/IcPgWuDD3N4_"
            },
            {
              "id": "481520",
              "name": "Altamash Faridi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/altamash-faridi-songs/OmA4hfy6YiM_"
            },
            {
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
            }
          ]
        },
        "songCount": 6,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "14279654",
        "name": "Kedarnath",
        "subtitle": "Amit Trivedi",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2019,
        "url": "https://www.jiosaavn.com/album/kedarnath/0VZPWDP3Hhs_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "457422",
              "name": "Amit Trivedi",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amit-trivedi-songs/BxN3kFKSp1o_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "457422",
              "name": "Amit Trivedi",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amit-trivedi-songs/BxN3kFKSp1o_"
            },
            {
              "id": "457465",
              "name": "Sumedha Karmahe",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sumedha-karmahe-songs/EfCd9qt6jNo_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "706985",
              "name": "Asees Kaur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/asees-kaur-songs/3LZ-9DxxiH8_"
            },
            {
              "id": "745460",
              "name": "Nikhita Gandhi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/nikhita-gandhi-songs/PP1RAOLWPAM_"
            },
            {
              "id": "457422",
              "name": "Amit Trivedi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amit-trivedi-songs/BxN3kFKSp1o_"
            },
            {
              "id": "653204",
              "name": "Dev Negi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dev-negi-songs/NpCqdI4dD5U_"
            }
          ]
        },
        "songCount": 7,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "1139549",
        "name": "Aashiqui 2",
        "subtitle": "Various Artists",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2013,
        "url": "https://www.jiosaavn.com/album/aashiqui-2/-iNdCmFNV9o_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455494",
              "name": "Various Artists",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/various-artists-songs/ztKx8IUBme8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "484852",
              "name": "Ankit Tiwari",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-tiwari-songs/eHCWDfgAqlg_"
            },
            {
              "id": "464912",
              "name": "Jeet Gannguli",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jeet-gannguli-songs/37adMqLwAgI_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "484852",
              "name": "Ankit Tiwari",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-tiwari-songs/eHCWDfgAqlg_"
            },
            {
              "id": "467309",
              "name": "Palak Muchhal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/palak-muchhal-songs/9RWBvFQhPxw_"
            },
            {
              "id": "455137",
              "name": "Tulsi Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tulsi-kumar-songs/31Xl5D0eU5Y_"
            },
            {
              "id": "455782",
              "name": "KK",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kk-songs/K,5uNVM,qpM_"
            },
            {
              "id": "462136",
              "name": "Mustafa Zahid",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mustafa-zahid-songs/JcSJqGjD54M_"
            },
            {
              "id": "455130",
              "name": "Shreya Ghoshal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
            }
          ]
        },
        "songCount": 11,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "31262422",
        "name": "Tadap",
        "subtitle": "Pritam",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2021,
        "url": "https://www.jiosaavn.com/album/tadap/LiUPUh2JIdE_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
            },
            {
              "id": "888127",
              "name": "Darshan Raval",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/darshan-raval-songs/a8luI6aaFas_"
            },
            {
              "id": "909299",
              "name": "Shashwat Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shashwat-singh-songs/gJthH4uEIuI_"
            },
            {
              "id": "460577",
              "name": "Charan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/charan-songs/1jooVizMjiI_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            },
            {
              "id": "788130",
              "name": "B Praak",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/b-praak-songs/CfABr-vmQdw_"
            },
            {
              "id": "3362043",
              "name": "Akashdeep Sengupta",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akashdeep-sengupta-songs/RDT4nQ,-jo8_"
            },
            {
              "id": "3837440",
              "name": "Dino James",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dino-james-songs/3bn-UnY9yGM_"
            },
            {
              "id": "464932",
              "name": "Neha Kakkar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
            },
            {
              "id": "455926",
              "name": "Javed Ali",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/javed-ali-songs/iJXgfejIuJQ_"
            },
            {
              "id": "505174",
              "name": "Mohammed Irfan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mohammed-irfan-songs/yGrHVvQrxok_"
            },
            {
              "id": "706985",
              "name": "Asees Kaur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/asees-kaur-songs/3LZ-9DxxiH8_"
            }
          ]
        },
        "songCount": 8,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      }
    ],
    "similarArtists": [],
    "isRadioPresent": true,
    "bio": "[{\"text\": \"Arijit Singh, is an Indian playback singer. He was one of top six contestants in reality-singing series, Fame Gurukul in 2005 and became an assistant to music director Pritam.Known for his soulful voice that reverberates with romantic songs, Arijit Singh is setting new trends in the music industry, by stirring up unforgetable notes for the young hearts. Arijit was born in Murshidabad, Jiaganj, West Bengal, India on 25th April, 1987. He got married to Koel(in Jan 2014)and the couple lives in Mumbai.\", \"title\": \"Introduction\", \"sequence\": 1}, {\"text\": \"Born to a Punjabi father and Bengali mother in Murshidabad in West Bengal, Arijit Singh, got trained in classical singing at an early age. He gives credit to his \\u00e2\\u20ac\\u02dcthree gurus\\u00e2\\u20ac\\u2122- Rajendra Prasad Hazari for teaching him Indian classical music, Dhirendra Prasad Hazari for tabla lessons, and Birendra Prasad Hazari for teaching him Rabindra sangeet and pop music. Also his grandmother sings, while his mother's sister (mausi) sings Indian Classical vocals. His maternal uncle plays the tabla.\\r\\n\\r\\nHis career as a playback singer came when he was in college and participated in a TV reality show called Fame Gurukul in 2005. Although he did not win the competition, he earned a large fan following and later went on to win \\u00e2\\u20ac\\u02dc10 Ke 10 Le Gaye Dil\\u00e2\\u20ac\\u2122 song competition reality show with audience votes.\\r\\n\\r\\nWhen TIPS industries signed him up for a contract when he was 18, he said, \\u00e2\\u20ac\\u0153I am very happy with this contract. They have promised us five albums in the next five years. But it doesn't mean that we cannot work outside the company. We just have to inform TIPS. For example, if I get an offer and decide to sing, I will have to inform the company about it. See, I am doing shows organized by other companies.\\\"\\r\\n\\r\\nIt was Shankar Ehsaan Loy who gave Arijit his break with the song All For One for the High School Musical 2 (Vol.1) Album. In that period, Arijit had also sung the title song Hum Hai Deewane (during which he says he slept through the auditions) for the Indian TV Show called Madhubala \\u00e2\\u20ac\\u201cEk Ishq Ek Junoon and Dadagiri with Avishek Das. The Show featured Sourav Ganguly. The other track he sang was for the Bengali TV serial Tomay Amay Mile, along with Ujjaini Mukherjee.\\r\\n\\r\\nHis moment of glory in Bollywood came with the song Tum Hi Ho for the romantic blockbuster Aashiqui 2 in 2013, which had topped charts worldwide and remained in the top 10 of Planet Bollywood for eight weeks, while also remaining on first spot on Top 20 of MTV India, for seven weeks. He also received the Filmfare Award for Best Male Playback Singer in the 59th Filmfare Awards for this song.\", \"title\": \"Early Career\", \"sequence\": 2}, {\"text\": \"Arijit Singh married his long time ladylove Koel Roy at Tarapith temple, West Bengal on January 20, 2014 in a private ceremony. The media got whiff of it only after he posted his wedding picture with his bride on his WhatsApp profile. His current wife, Koel has a four-year-old daughter from her earlier marriage, who is reportedly fond of Arijit. Reports say that Arijit Singh\\u00e2\\u20ac\\u2122s first marriage was with one of his co-contestants from a reality show.\", \"title\": \"His Marriage\", \"sequence\": 3}, {\"text\": \"1. Tum Hi Ho - 2013\\r\\n2. Kabhi Jo Baadal Barse - 2013\\r\\n3. Sawan Aaya Hai - 2014\\r\\n4. Raabta (Siyaah Raatein) - 2012\\r\\n5. Hamari Adhuri Kahani - 2015\\r\\n6. Humdard - 2014\\r\\n7. Manwa Laage - 2014\\r\\n8. Phir Mohabbat - 2011\\r\\n9. Aaj Phir - 2014\\r\\n10. Suno Na Sangemarmar - 2014\", \"title\": \"Top 10 Songs\", \"sequence\": 4}]",
    "dob": "25-04-1987",
    "fb": "",
    "twitter": "",
    "wiki": "http://en.wikipedia.org/wiki/Arijit_Singh",
    "urls": {
      "albums": "https://www.jiosaavn.com/artist/arijit-singh-albums/LlRWpHzy3Hk_",
      "bio": "/artist/arijit-singh-bio/LlRWpHzy3Hk_",
      "comments": "https://www.jiosaavn.com/artist/arijit-singh-comments/LlRWpHzy3Hk_",
      "songs": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
    },
    "availableLanguages": [
      "hindi",
      "bengali",
      "telugu",
      "english",
      "unknown",
      "punjabi",
      "kannada",
      "marathi",
      "tamil",
      "bhojpuri"
    ],
    "fanCount": 20716607,
    "isFollowed": false
  }
}
```

+++

|          **Query Parameter**          | **Description** |              **Required**              |
| :-----------------------------------: | :-------------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] |    Artist ID    | [!badge variant="primary" text="True"] |

## Artist details by link

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/artist?link=https://www.jiosaavn.com/artist/arijit-singh-/LlRWpHzy3Hk_
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/artist?link=https://www.jiosaavn.com/artist/arijit-singh-/LlRWpHzy3Hk_' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Artist details fetched successfully!",
  "data": {
    "id": "459320",
    "name": "Arijit Singh",
    "subtitle": "Artist • 20716607 Listeners",
    "image": [
      {
        "quality": "50x50",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_50x50.jpg"
      },
      {
        "quality": "150x150",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg"
      },
      {
        "quality": "500x500",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_500x500.jpg"
      }
    ],
    "followerCount": 14584087,
    "type": "artist",
    "isVerified": true,
    "dominantLanguage": "hindi",
    "dominantType": "singer",
    "topSongs": [
      {
        "id": "qZtKBMZ_",
        "name": "Apna Bana Le",
        "subtitle": "Arijit Singh, Sachin-Jigar - Bhediya",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/apna-bana-le/ATIfejZ9bWw",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2022,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Sachin-Jigar",
        "song": null,
        "albumId": "38682222",
        "album": "Bhediya",
        "label": "Zee Music Co.",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDylA8lQwDowJF0rwulZQb41oGFWui//Ii/9d8Rp0MdqrAGAnyTSi1MVxw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/bhediya/wSM2AOubajk_",
        "duration": 261,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Jag Ki Hirasat Se",
        "starred": false,
        "copyrightText": "© 2023 Zee Music Company",
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
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
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
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Sachin-Jigar_002_20180507092234_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
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
              "id": "511656",
              "name": "Varun Dhawan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Varun_Dhawan_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/varun-dhawan-songs/nPLp3EdUdaA_"
            },
            {
              "id": "701752",
              "name": "Kriti Sanon",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Kriti_Sanon_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kriti-sanon-songs/9UtnOColzrU_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "_rJmbKSP",
        "name": "Shayad",
        "subtitle": "Pritam, Arijit Singh - Love Aaj Kal (Original Motion Picture Soundtrack)",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/shayad/LxohXBZ7ZGM",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/862/Love-Aaj-Kal-Hindi-2020-20200214140423-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2020,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "19072107",
        "album": "Love Aaj Kal (Original Motion Picture Soundtrack)",
        "label": "Sony Music Entertainment India Pvt. Ltd.",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyT19PzAX7YyzgkHYWQybH7xN4MJMIg+vv6YsG3yMFglt8bvlJKKDQQRw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/love-aaj-kal-original-motion-picture-soundtrack/08dQgBZGh20_",
        "duration": 247,
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
        "copyrightText": "(P) 2020 Sony Music Entertainment India Pvt. Ltd.",
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
              "id": "456259",
              "name": "Irshad Kamil",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Irshad_Kamil_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
            },
            {
              "id": "5607405",
              "name": "Sara Ali Khan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Sara_Ali_Khan_20200303113945_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sara-ali-khan-songs/R8Muu,CMC78_"
            },
            {
              "id": "3392054",
              "name": "Kartik Aaryan",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Kartik_Aaryan_000_20211119092218_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kartik-aaryan-songs/s6R4Iv6pc3s_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": null
      },
      {
        "id": "gTOlL2wh",
        "name": "Sawan Aaya Hai",
        "subtitle": "Tony Kakkar, Mithoon, Arijit Singh - Creature 3D",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/sawan-aaya-hai/FzwkXTgCQFs",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/244/Creature-3D-Hindi-2014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2014,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Tony Kakkar, Mithoon",
        "song": null,
        "albumId": "1194579",
        "album": "Creature 3D",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyev1qlIfUK4igRCF4+gjCLkq7zU4uI/aYhVaTB/dNYoVHm+0ney/T5xw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/creature-3d/TDDY3rBIUiw_",
        "duration": 288,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Sawan Aaya Hai-Lyrics",
        "starred": false,
        "copyrightText": "©  2014 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
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
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Tony_Kakkar_004_20220519082733_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
            {
              "id": "456317",
              "name": "Bipasha Basu",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Bipasha_Basu_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/bipasha-basu-songs/CxaBGYXiR00_"
            },
            {
              "id": "740160",
              "name": "Imran Abbas Naqvi",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/imran-abbas-naqvi-songs/vGzqu5QGfaM_"
            },
            {
              "id": "740161",
              "name": "Bikramjeet Singh",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/bikramjeet-singh-songs/r,TxXGw7xLM_"
            },
            {
              "id": "468113",
              "name": "Deepraj Rana",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/deepraj-rana-songs/0cjTGgkRDAg_"
            },
            {
              "id": "466125",
              "name": "Mukul Dev",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mukul-dev-songs/n,RRVe0ndHs_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "2aaBp3ZE",
        "name": "Tumse Bhi Zyada",
        "subtitle": "Arijit Singh - Tadap",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/tumse-bhi-zyada/QgkKcwQDbXY",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/439/Tadap-Hindi-2021-20211203151001-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2021,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Pritam",
        "song": null,
        "albumId": "31262422",
        "album": "Tadap",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyGKfyjN+uwn9R72miHHSmmJ8iv1wv7QxfLFr1Qn7gNVsAe+/0pLQwdRw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/tadap/LiUPUh2JIdE_",
        "duration": 319,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "tumse bhee zyaada tumse pyaar kiya",
        "starred": false,
        "copyrightText": "℗ 2021 Super Cassettes Industries Private Limited",
        "artistMap": {
          "primaryArtists": [
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
              "id": "458223",
              "name": "Ashish Pandit",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/515/Love-Shagun-Hindi-2016-20191118095603-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ashish-pandit-songs/VhzkfC-tgFg_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "aRZbUYD7",
        "name": "Tum Hi Ho",
        "subtitle": "Arijit Singh - Aashiqui 2",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/tum-hi-ho/EToxUyFpcwQ",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/430/Aashiqui-2-Hindi-2013-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2013,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon",
        "song": null,
        "albumId": "1139549",
        "album": "Aashiqui 2",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyRofh8YQPTqjR2tIxTYXqGEQHgqnNfsetouo4rsD0s4RZKObWAyHrEhw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/aashiqui-2/-iNdCmFNV9o_",
        "duration": 262,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "Tere Bina Kya Wajood Mera",
        "starred": false,
        "copyrightText": "©  2013 ",
        "artistMap": {
          "primaryArtists": [
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
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
              "id": "477854",
              "name": "Shraddha Kapoor",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Shraddha_Kapoor_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shraddha-kapoor-songs/97qRLkdq3gI_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "riNBfJ3P",
        "name": "Sanam Re",
        "subtitle": "Arijit Singh, Mithoon - Sanam Re",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/sanam-re/AgElcxJ6BGM",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/829/Sanam-Re-Hindi-2015-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2016,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Mithoon",
        "song": null,
        "albumId": "1699057",
        "album": "Sanam Re",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyrlecNtdN+6iCR2CebRPf8Tf/C/UqCdBhzkHfD38oIA4fPHKDwljT6hw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/sanam-re/usTMyKvgz4M_",
        "duration": 308,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "sanam re, sanam re, too mera sanam hua  re",
        "starred": false,
        "copyrightText": "©  2016 ",
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
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
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
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "singer",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "lyricist",
              "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "531878",
              "name": "Pulkit Samrat",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pulkit-samrat-songs/XNeUtvztuY8_"
            },
            {
              "id": "505299",
              "name": "Yami Gautam",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Yami_Gautam_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/yami-gautam-songs/hK7MoHPut2M_"
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
              "id": "679274",
              "name": "Urvashi Rautela",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Urvashi_Rautela_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/urvashi-rautela-songs/GcTwpCvBIMs_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      },
      {
        "id": "lteSMrqT",
        "name": "Main Dhoondne Ko Zamaane Mein",
        "subtitle": "Gaurav Dagaonkar, Arijit Singh - Heartless",
        "type": "song",
        "url": "https://www.jiosaavn.com/song/main-dhoondne-ko-zamaane-mein/HBwOYjlCRmc",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/112/Heartless-2014-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/112/Heartless-2014-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/112/Heartless-2014-500x500.jpg"
          }
        ],
        "language": "hindi",
        "year": 2014,
        "playCount": 0,
        "explicitContent": false,
        "listCount": 0,
        "listType": "",
        "list": "",
        "music": "Gaurav Dagaonkar",
        "song": null,
        "albumId": "1148662",
        "album": "Heartless",
        "label": "",
        "origin": "song",
        "isDolbyContent": false,
        "320kbps": true,
        "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDypARW5XI2blkziZ7sOnVcYvG5EN90llxl7glNPThVZzuw4CBAvu1A+Rw7tS9a8Gtq",
        "albumUrl": "https://www.jiosaavn.com/album/heartless/oSeU3sTMFgI_",
        "duration": 263,
        "rights": {
          "code": "0",
          "cacheable": "true",
          "delete_cached_object": "false",
          "reason": ""
        },
        "cacheState": "false",
        "hasLyrics": true,
        "lyricsSnippet": "main dhoondhane ko zamaane mein jab vafaa nikalaa",
        "starred": false,
        "copyrightText": "©  2014 ",
        "artistMap": {
          "primaryArtists": [
            {
              "id": "490085",
              "name": "Gaurav Dagaonkar",
              "role": "primary_artists",
              "image": "https://c.saavncdn.com/artists/Gaurav_Dagaonkar_20191216113105_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gaurav-dagaonkar-songs/XPxdM,JR8tY_"
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
              "id": "490085",
              "name": "Gaurav Dagaonkar",
              "role": "music",
              "image": "https://c.saavncdn.com/artists/Gaurav_Dagaonkar_20191216113105_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gaurav-dagaonkar-songs/XPxdM,JR8tY_"
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
              "id": "469700",
              "name": "Arafat Mehmood",
              "role": "lyricist",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arafat-mehmood-songs/7JfoXUgI2OM_"
            },
            {
              "id": "471684",
              "name": "Adhyayan Suman",
              "role": "starring",
              "image": "https://c.saavncdn.com/446/Soniyo-Version-2-0--Hindi-2019-20190430113919-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/adhyayan-suman-songs/ngbwwd6Sekc_"
            },
            {
              "id": "684210",
              "name": "Arianaayam",
              "role": "starring",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arianaayam-songs/-nToxkJP6JA_"
            },
            {
              "id": "455529",
              "name": "Om Puri",
              "role": "starring",
              "image": "https://c.saavncdn.com/artists/Om_Puri_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/om-puri-songs/Naa70mb2fgE_"
            },
            {
              "id": "461019",
              "name": "Deepti Naval",
              "role": "starring",
              "image": "https://c.saavncdn.com/000/default_Saregama_150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/deepti-naval-songs/BtVBt-YTiJk_"
            },
            {
              "id": "461079",
              "name": "Shekhar Suman",
              "role": "starring",
              "image": "https://c.saavncdn.com/728/Shukriya-2004-150x150.jpg",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-suman-songs/PsCGUekUNCY_"
            }
          ]
        },
        "releaseDate": "",
        "trillerAvailable": false,
        "lyricsId": ""
      }
    ],
    "topAlbums": [
      {
        "id": "47279722",
        "name": "Gadar 2",
        "subtitle": "Mithoon, Uttam Singh",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/gadar-2/zKU0zGJl3GY_",
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
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "461323",
              "name": "Uttam Singh",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/uttam-singh-songs/oNUhFwf,KLc_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "702592",
              "name": "Mithoon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
            },
            {
              "id": "455127",
              "name": "Udit Narayan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_"
            },
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
            },
            {
              "id": "461323",
              "name": "Uttam Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/uttam-singh-songs/oNUhFwf,KLc_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "459733",
              "name": "Aditya Narayan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/aditya-narayan-songs/TY,T7F3OI64_"
            },
            {
              "id": "455636",
              "name": "Sukhwinder Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukhwinder-singh-songs/X04Pj7GRt40_"
            },
            {
              "id": "531639",
              "name": "Neeti Mohan",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neeti-mohan-songs/3rVrdWgQlqs_"
            },
            {
              "id": "702452",
              "name": "Vishal Mishra",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-mishra-songs/f0sXoS0mUnE_"
            },
            {
              "id": "593966",
              "name": "Shehnaz Akhtar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shehnaz-akhtar-songs/2ToejJKEHDM_"
            },
            {
              "id": "684962",
              "name": "Sahil Akhtar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sahil-akhtar-songs/KPyKn19Z7i0_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            }
          ]
        },
        "songCount": 7,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "38682222",
        "name": "Bhediya",
        "subtitle": "Sachin-Jigar",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2022,
        "url": "https://www.jiosaavn.com/album/bhediya/wSM2AOubajk_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/222/Bhediya-Hindi-2022-20230616085155-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            },
            {
              "id": "640931",
              "name": "Sachin Sanghvi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-sanghvi-songs/2jDoqGtKfFE_"
            },
            {
              "id": "470069",
              "name": "Jigar Saraiya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jigar-saraiya-songs/1uFvGEnOrnI_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "461968",
              "name": "Sachin-Jigar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sachin-jigar-songs/JO1Nx088Pfo_"
            },
            {
              "id": "4583855",
              "name": "Sakshi Holkar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sakshi-holkar-songs/wsANapvxb9Y_"
            },
            {
              "id": "482886",
              "name": "Siddharth Basrur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/siddharth-basrur-songs/uDwDFPrgbjI_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "455636",
              "name": "Sukhwinder Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukhwinder-singh-songs/X04Pj7GRt40_"
            },
            {
              "id": "3712926",
              "name": "Rashmeet Kaur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/rashmeet-kaur-songs/xQ7jDdbDGKw_"
            },
            {
              "id": "459583",
              "name": "Ash King",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ash-king-songs/V61FW0l-DEU_"
            },
            {
              "id": "473376",
              "name": "Divya Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/divya-kumar-songs/7oZnjNxjuR4_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            }
          ]
        },
        "songCount": 5,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "41039709",
        "name": "Pathaan",
        "subtitle": "Vishal &amp; Shekhar, Sanchit Balhara, Ankit Balhara",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2022,
        "url": "https://www.jiosaavn.com/album/pathaan/kw5AWtM1BZk_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/807/Pathaan-Hindi-2022-20221222104158-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
            },
            {
              "id": "672066",
              "name": "Sanchit Balhara",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sanchit-balhara-songs/34VS9v3X77U_"
            },
            {
              "id": "672069",
              "name": "Ankit Balhara",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-balhara-songs/AtNj5y6iSWI_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "459880",
              "name": "Vishal &amp; Shekhar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
            },
            {
              "id": "672066",
              "name": "Sanchit Balhara",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sanchit-balhara-songs/34VS9v3X77U_"
            },
            {
              "id": "672069",
              "name": "Ankit Balhara",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ankit-balhara-songs/AtNj5y6iSWI_"
            },
            {
              "id": "455148",
              "name": "Shilpa Rao",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shilpa-rao-songs/IVd,BmJX7sA_"
            },
            {
              "id": "461068",
              "name": "Caralisa Monteiro",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/caralisa-monteiro-songs/U4CwqcIkmsU_"
            },
            {
              "id": "455669",
              "name": "Vishal Dadlani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
            },
            {
              "id": "459883",
              "name": "Shekhar Ravjiani",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "670650",
              "name": "Sukriti Kakar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sukriti-kakar-songs/fB0XSQjRbtM_"
            },
            {
              "id": "4139728",
              "name": "Magdalena Supel",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/magdalena-supel-songs/ibMwOg5tQCo_"
            },
            {
              "id": "14921580",
              "name": "Maanya Narang",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/maanya-narang-songs/iMNTXvX1PWU_"
            },
            {
              "id": "7662618",
              "name": "Riya Duggal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/riya-duggal-songs/mmN5pW2L8fI_"
            }
          ]
        },
        "songCount": 4,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "43600311",
        "name": "Tu Jhoothi Main Makkaar",
        "subtitle": "Pritam",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/tu-jhoothi-main-makkaar/Zk6,htMIngg_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/903/Tu-Jhoothi-Main-Makkaar-Hindi-2023-20230316165419-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "745460",
              "name": "Nikhita Gandhi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/nikhita-gandhi-songs/PP1RAOLWPAM_"
            },
            {
              "id": "458681",
              "name": "Amitabh Bhattacharya",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
            },
            {
              "id": "460577",
              "name": "Charan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/charan-songs/1jooVizMjiI_"
            },
            {
              "id": "455129",
              "name": "Sunidhi Chauhan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-songs/ZIyMwJzHJwI_"
            },
            {
              "id": "909299",
              "name": "Shashwat Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shashwat-singh-songs/gJthH4uEIuI_"
            },
            {
              "id": "881158",
              "name": "Jubin Nautiyal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
            },
            {
              "id": "455130",
              "name": "Shreya Ghoshal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
            },
            {
              "id": "473376",
              "name": "Divya Kumar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/divya-kumar-songs/7oZnjNxjuR4_"
            }
          ]
        },
        "songCount": 6,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "30775211",
        "name": "Sooryavanshi",
        "subtitle": "Various Artists",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2021,
        "url": "https://www.jiosaavn.com/album/sooryavanshi/q3wQulyDlhg_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/415/Sooryavanshi-Hindi-2021-20211111121001-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "455494",
              "name": "Various Artists",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/various-artists-songs/ztKx8IUBme8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "3953689",
              "name": "Lijo George-Dj Chetas",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/lijo-george-dj-chetas-songs/OUMjvsOYLbw_"
            },
            {
              "id": "456323",
              "name": "Pritam",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
            },
            {
              "id": "1595701",
              "name": "Tanishk Bagchi",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanishk-bagchi-songs/Y-5m-phldpg_"
            },
            {
              "id": "458188",
              "name": "Kaushik",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kaushik-songs/uHkINkejJZs_"
            },
            {
              "id": "465738",
              "name": "Guddu",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/guddu-songs/C2MlvJiYwYc_"
            },
            {
              "id": "462319",
              "name": "Akash",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/akash-songs/z5o8fN,xGHA_"
            },
            {
              "id": "658557",
              "name": "Pav Dharia",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pav-dharia-songs/FO8vGJq0VCQ_"
            },
            {
              "id": "457905",
              "name": "Viju Shah",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/viju-shah-songs/72XE57yXd5E_"
            },
            {
              "id": "1393898",
              "name": "Lijo George",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/lijo-george-songs/BLEppbJmz9E_"
            },
            {
              "id": "455652",
              "name": "Daler Mehndi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/daler-mehndi-songs/00FjmqGw0V8_"
            },
            {
              "id": "459320",
              "name": "Arijit Singh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
            },
            {
              "id": "531639",
              "name": "Neeti Mohan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neeti-mohan-songs/3rVrdWgQlqs_"
            },
            {
              "id": "658557",
              "name": "Pav Dharia",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/pav-dharia-songs/FO8vGJq0VCQ_"
            },
            {
              "id": "745460",
              "name": "Nikhita Gandhi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/nikhita-gandhi-songs/PP1RAOLWPAM_"
            },
            {
              "id": "455127",
              "name": "Udit Narayan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_"
            },
            {
              "id": "455120",
              "name": "Alka Yagnik",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
            }
          ]
        },
        "songCount": 5,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      }
    ],
    "similarArtists": [],
    "isRadioPresent": true,
    "bio": "[{\"text\": \"Arijit Singh, is an Indian playback singer. He was one of top six contestants in reality-singing series, Fame Gurukul in 2005 and became an assistant to music director Pritam.Known for his soulful voice that reverberates with romantic songs, Arijit Singh is setting new trends in the music industry, by stirring up unforgetable notes for the young hearts. Arijit was born in Murshidabad, Jiaganj, West Bengal, India on 25th April, 1987. He got married to Koel(in Jan 2014)and the couple lives in Mumbai.\", \"title\": \"Introduction\", \"sequence\": 1}, {\"text\": \"Born to a Punjabi father and Bengali mother in Murshidabad in West Bengal, Arijit Singh, got trained in classical singing at an early age. He gives credit to his \\u00e2\\u20ac\\u02dcthree gurus\\u00e2\\u20ac\\u2122- Rajendra Prasad Hazari for teaching him Indian classical music, Dhirendra Prasad Hazari for tabla lessons, and Birendra Prasad Hazari for teaching him Rabindra sangeet and pop music. Also his grandmother sings, while his mother's sister (mausi) sings Indian Classical vocals. His maternal uncle plays the tabla.\\r\\n\\r\\nHis career as a playback singer came when he was in college and participated in a TV reality show called Fame Gurukul in 2005. Although he did not win the competition, he earned a large fan following and later went on to win \\u00e2\\u20ac\\u02dc10 Ke 10 Le Gaye Dil\\u00e2\\u20ac\\u2122 song competition reality show with audience votes.\\r\\n\\r\\nWhen TIPS industries signed him up for a contract when he was 18, he said, \\u00e2\\u20ac\\u0153I am very happy with this contract. They have promised us five albums in the next five years. But it doesn't mean that we cannot work outside the company. We just have to inform TIPS. For example, if I get an offer and decide to sing, I will have to inform the company about it. See, I am doing shows organized by other companies.\\\"\\r\\n\\r\\nIt was Shankar Ehsaan Loy who gave Arijit his break with the song All For One for the High School Musical 2 (Vol.1) Album. In that period, Arijit had also sung the title song Hum Hai Deewane (during which he says he slept through the auditions) for the Indian TV Show called Madhubala \\u00e2\\u20ac\\u201cEk Ishq Ek Junoon and Dadagiri with Avishek Das. The Show featured Sourav Ganguly. The other track he sang was for the Bengali TV serial Tomay Amay Mile, along with Ujjaini Mukherjee.\\r\\n\\r\\nHis moment of glory in Bollywood came with the song Tum Hi Ho for the romantic blockbuster Aashiqui 2 in 2013, which had topped charts worldwide and remained in the top 10 of Planet Bollywood for eight weeks, while also remaining on first spot on Top 20 of MTV India, for seven weeks. He also received the Filmfare Award for Best Male Playback Singer in the 59th Filmfare Awards for this song.\", \"title\": \"Early Career\", \"sequence\": 2}, {\"text\": \"Arijit Singh married his long time ladylove Koel Roy at Tarapith temple, West Bengal on January 20, 2014 in a private ceremony. The media got whiff of it only after he posted his wedding picture with his bride on his WhatsApp profile. His current wife, Koel has a four-year-old daughter from her earlier marriage, who is reportedly fond of Arijit. Reports say that Arijit Singh\\u00e2\\u20ac\\u2122s first marriage was with one of his co-contestants from a reality show.\", \"title\": \"His Marriage\", \"sequence\": 3}, {\"text\": \"1. Tum Hi Ho - 2013\\r\\n2. Kabhi Jo Baadal Barse - 2013\\r\\n3. Sawan Aaya Hai - 2014\\r\\n4. Raabta (Siyaah Raatein) - 2012\\r\\n5. Hamari Adhuri Kahani - 2015\\r\\n6. Humdard - 2014\\r\\n7. Manwa Laage - 2014\\r\\n8. Phir Mohabbat - 2011\\r\\n9. Aaj Phir - 2014\\r\\n10. Suno Na Sangemarmar - 2014\", \"title\": \"Top 10 Songs\", \"sequence\": 4}]",
    "dob": "25-04-1987",
    "fb": "",
    "twitter": "",
    "wiki": "http://en.wikipedia.org/wiki/Arijit_Singh",
    "urls": {
      "albums": "https://www.jiosaavn.com/artist/arijit-singh-albums/LlRWpHzy3Hk_",
      "bio": "/artist/arijit-singh-bio/LlRWpHzy3Hk_",
      "comments": "https://www.jiosaavn.com/artist/arijit-singh-comments/LlRWpHzy3Hk_",
      "songs": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
    },
    "availableLanguages": [
      "hindi",
      "bengali",
      "telugu",
      "english",
      "unknown",
      "punjabi",
      "kannada",
      "marathi",
      "tamil",
      "bhojpuri"
    ],
    "fanCount": 20716607,
    "isFollowed": false
  }
}
```

+++

|           **Query Parameter**           |            **Description**            |              **Required**              |
| :-------------------------------------: | :-----------------------------------: | :------------------------------------: |
| [!badge variant="contrast" text="link"] | Song link from <https://jiosaavn.com> | [!badge variant="primary" text="True"] |

## Artist Songs by artist ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/artist/songs?id=459320
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/artist/songs?id=459320' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Artist songs fetched successfully!",
  "data": {
    "id": "459320",
    "name": "Arijit Singh",
    "image": [
      {
        "quality": "50x50",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_50x50.jpg"
      },
      {
        "quality": "150x150",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg"
      },
      {
        "quality": "500x500",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_500x500.jpg"
      }
    ],
    "followerCount": 14584087,
    "type": "artist",
    "isVerified": true,
    "dominantLanguage": "hindi",
    "dominantType": "singer",
    "topSongs": {
      "total": 2414,
      "last_page": false,
      "songs": [
        {
          "id": "SmZzM21e",
          "name": "Neeve Na Neeve Na",
          "subtitle": "Sandeep Chowta, Neha Kakkar, Arijit Singh - Kedi",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/neeve-na-neeve-na/IwUxSzkCBlY",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/369/Kedi-2010-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/369/Kedi-2010-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/369/Kedi-2010-500x500.jpg"
            }
          ],
          "language": "telugu",
          "year": 2010,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Sandeep Chowta",
          "song": null,
          "albumId": "1034369",
          "album": "Kedi",
          "label": "Aditya Music",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyW5dDmMR1Eqd3cqiYO+tBES0gZ4nTwNt1EJ86NYEdBSFEbR5NMjxMgxw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/kedi/,un5mYLHDao_",
          "duration": 335,
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
          "copyrightText": "©  2010 Aditya Music",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "458154",
                "name": "Sandeep Chowta",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/artists/Sandeep_Chowta_000_20210616180844_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandeep-chowta-songs/PACF9hBtu3M_"
              },
              {
                "id": "464932",
                "name": "Neha Kakkar",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/artists/Neha_Kakkar_006_20200822042626_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "458154",
                "name": "Sandeep Chowta",
                "role": "music",
                "image": "http://c.saavncdn.com/artists/Sandeep_Chowta_000_20210616180844_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandeep-chowta-songs/PACF9hBtu3M_"
              },
              {
                "id": "464932",
                "name": "Neha Kakkar",
                "role": "singer",
                "image": "http://c.saavncdn.com/artists/Neha_Kakkar_006_20200822042626_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singer",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "458654",
                "name": "Chinni Charan",
                "role": "lyricist",
                "image": "http://c.saavncdn.com/749/Nenu-Naa-Friends-Telugu-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/chinni-charan-songs/EeBqsSS,k,E_"
              },
              {
                "id": "457221",
                "name": "Nagarjuna",
                "role": "starring",
                "image": "http://c.saavncdn.com/artists/Nagarjuna_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nagarjuna-songs/er92DnwuINI_"
              },
              {
                "id": "466672",
                "name": "Mamata Mohandas",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mamata-mohandas-songs/L1MWh2Z-beo_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "fPjUXN9Q",
          "name": "Shukr Tera",
          "subtitle": "Arijit Singh, Chinmayi Sripada - Love In Winter",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/shukr-tera/FjgBZCx,DmI",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-500x500.jpg"
            }
          ],
          "language": "hindi",
          "year": 1999,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Mithoon",
          "song": null,
          "albumId": "11837203",
          "album": "Love In Winter",
          "label": "Rajshri",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyUvhiC31ZNb576afUJnSBwqq+nxfiHByc/FDeaouAUHuOeMVhfevOZRw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/love-in-winter/ZIO5syEo90w_",
          "duration": 361,
          "rights": {
            "code": "0",
            "cacheable": "true",
            "delete_cached_object": "false",
            "reason": ""
          },
          "cacheState": "false",
          "hasLyrics": true,
          "lyricsSnippet": "Iss mohabbat ka harjana kis tarah bharu main",
          "starred": false,
          "copyrightText": "©  1999 Rajshri",
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
                "id": "702592",
                "name": "Mithoon",
                "role": "lyricist",
                "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": ""
        },
        {
          "id": "6389yMfG",
          "name": "E Akash",
          "subtitle": "Shaan - Tobu Aashte Hobe Phire",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/e-akash/RltTCA19UXQ",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2005,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Arijit Singh",
          "song": null,
          "albumId": "31739288",
          "album": "Tobu Aashte Hobe Phire",
          "label": "Echo Entertainment Pvt. Ltd.",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyFGF+qbHooQ4hmhjsCabBfFjIv8B4P8BQOgFTHTuH86RWAPYosn0rNxw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "duration": 276,
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
          "copyrightText": "© 2005 Echo Entertainment Pvt. Ltd.",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "455135",
                "name": "Shaan",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "455135",
                "name": "Shaan",
                "role": "singer",
                "image": "http://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "752807",
                "name": "Kaushik Chakraborty",
                "role": "lyricist",
                "image": "http://c.saavncdn.com/892/Baishnav-Jan-To-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kaushik-chakraborty-songs/5onpzEUYUXg_"
              },
              {
                "id": "898866",
                "name": "Kakoli",
                "role": "starring",
                "image": "http://c.saavncdn.com/584/Jab-Koi-Baat-Single-Hindi-2019-20190205041406-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kakoli-songs/dGdK9J3HzVs_"
              },
              {
                "id": "4999051",
                "name": "Sirin",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sirin-songs/H6EPoT0Boww_"
              },
              {
                "id": "855150",
                "name": "Shilajit",
                "role": "starring",
                "image": "http://c.saavncdn.com/038/Amra-Bengali-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilajit-songs/uvCPGIZ4hyw_"
              },
              {
                "id": "460250",
                "name": "Kanchan",
                "role": "starring",
                "image": "http://c.saavncdn.com/744/Tu-Meri-Rani-Banja-Hindi-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kanchan-songs/aY1OMEWgZnk_"
              },
              {
                "id": "787069",
                "name": "Milan",
                "role": "starring",
                "image": "http://c.saavncdn.com/493/Rewind-English-2017-20180608155233-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/milan-songs/TyLkRYqbZt4_"
              },
              {
                "id": "579296",
                "name": "Soma",
                "role": "starring",
                "image": "http://c.saavncdn.com/199/Amar-Moner-Koney-Bengali-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soma-songs/ZEzZRGQRTys_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "jtkuvgoa",
          "name": "Shono Kaan Pete",
          "subtitle": "Nipobithi, Anusua - Tobu Aashte Hobe Phire",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/shono-kaan-pete/GhwARAJXWFI",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2005,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Arijit Singh",
          "song": null,
          "albumId": "31739288",
          "album": "Tobu Aashte Hobe Phire",
          "label": "Echo Entertainment Pvt. Ltd.",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyoLtwKVLbOeFdlwfn/Pt2cvvqGdmXx9mo/a6nyB8QfeDUncU13cZtWxw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "duration": 233,
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
          "copyrightText": "© 2005 Echo Entertainment Pvt. Ltd.",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "817440",
                "name": "Nipobithi",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nipobithi-songs/G,po7l7HZe0_"
              },
              {
                "id": "12106882",
                "name": "Anusua",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "817440",
                "name": "Nipobithi",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nipobithi-songs/G,po7l7HZe0_"
              },
              {
                "id": "12106882",
                "name": "Anusua",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "752807",
                "name": "Kaushik Chakraborty",
                "role": "lyricist",
                "image": "http://c.saavncdn.com/892/Baishnav-Jan-To-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kaushik-chakraborty-songs/5onpzEUYUXg_"
              },
              {
                "id": "898866",
                "name": "Kakoli",
                "role": "starring",
                "image": "http://c.saavncdn.com/584/Jab-Koi-Baat-Single-Hindi-2019-20190205041406-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kakoli-songs/dGdK9J3HzVs_"
              },
              {
                "id": "4999051",
                "name": "Sirin",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sirin-songs/H6EPoT0Boww_"
              },
              {
                "id": "855150",
                "name": "Shilajit",
                "role": "starring",
                "image": "http://c.saavncdn.com/038/Amra-Bengali-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilajit-songs/uvCPGIZ4hyw_"
              },
              {
                "id": "460250",
                "name": "Kanchan",
                "role": "starring",
                "image": "http://c.saavncdn.com/744/Tu-Meri-Rani-Banja-Hindi-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kanchan-songs/aY1OMEWgZnk_"
              },
              {
                "id": "787069",
                "name": "Milan",
                "role": "starring",
                "image": "http://c.saavncdn.com/493/Rewind-English-2017-20180608155233-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/milan-songs/TyLkRYqbZt4_"
              },
              {
                "id": "579296",
                "name": "Soma",
                "role": "starring",
                "image": "http://c.saavncdn.com/199/Amar-Moner-Koney-Bengali-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soma-songs/ZEzZRGQRTys_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "XRV4hqU0",
          "name": "Jharo Jharo",
          "subtitle": "Shaan, Subhomita, Arijit Singh, Anusua, Indrajit - Tobu Aashte Hobe Phire",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/jharo-jharo/KDo9BRxBYgM",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2005,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Arijit Singh",
          "song": null,
          "albumId": "31739288",
          "album": "Tobu Aashte Hobe Phire",
          "label": "Echo Entertainment Pvt. Ltd.",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy7zy16v/EkcaEVzlFrQRHl5956lXXSQkreoAo1r8ZvVfVjfDG3WD7gxw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "duration": 72,
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
          "copyrightText": "© 2005 Echo Entertainment Pvt. Ltd.",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "455135",
                "name": "Shaan",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "685544",
                "name": "Subhomita",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/719/Tomaye-Bhalobeshe-Bengali-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomita-songs/gnA3DfCi9ac_"
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
                "id": "12106882",
                "name": "Anusua",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "457960",
                "name": "Indrajit",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/430/Agnipath-2005-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/indrajit-songs/zZPkC2HY7TE_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "455135",
                "name": "Shaan",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "685544",
                "name": "Subhomita",
                "role": "singer",
                "image": "https://c.saavncdn.com/719/Tomaye-Bhalobeshe-Bengali-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomita-songs/gnA3DfCi9ac_"
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
                "id": "12106882",
                "name": "Anusua",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "457960",
                "name": "Indrajit",
                "role": "singer",
                "image": "https://c.saavncdn.com/430/Agnipath-2005-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/indrajit-songs/zZPkC2HY7TE_"
              },
              {
                "id": "752807",
                "name": "Kaushik Chakraborty",
                "role": "lyricist",
                "image": "https://c.saavncdn.com/892/Baishnav-Jan-To-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kaushik-chakraborty-songs/5onpzEUYUXg_"
              },
              {
                "id": "898866",
                "name": "Kakoli",
                "role": "starring",
                "image": "https://c.saavncdn.com/584/Jab-Koi-Baat-Single-Hindi-2019-20190205041406-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kakoli-songs/dGdK9J3HzVs_"
              },
              {
                "id": "4999051",
                "name": "Sirin",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sirin-songs/H6EPoT0Boww_"
              },
              {
                "id": "855150",
                "name": "Shilajit",
                "role": "starring",
                "image": "https://c.saavncdn.com/038/Amra-Bengali-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilajit-songs/uvCPGIZ4hyw_"
              },
              {
                "id": "460250",
                "name": "Kanchan",
                "role": "starring",
                "image": "https://c.saavncdn.com/744/Tu-Meri-Rani-Banja-Hindi-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kanchan-songs/aY1OMEWgZnk_"
              },
              {
                "id": "787069",
                "name": "Milan",
                "role": "starring",
                "image": "https://c.saavncdn.com/493/Rewind-English-2017-20180608155233-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/milan-songs/TyLkRYqbZt4_"
              },
              {
                "id": "579296",
                "name": "Soma",
                "role": "starring",
                "image": "https://c.saavncdn.com/199/Amar-Moner-Koney-Bengali-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soma-songs/ZEzZRGQRTys_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "lzNM1y9t",
          "name": "Bhalobasa",
          "subtitle": "Shaan, Subhomita, Arijit Singh, Anusua, Indrajit - Tobu Aashte Hobe Phire",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/bhalobasa/HBIlfEVJDkc",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2005,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Arijit Singh",
          "song": null,
          "albumId": "31739288",
          "album": "Tobu Aashte Hobe Phire",
          "label": "Echo Entertainment Pvt. Ltd.",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyw3Lpra/UrwVn2GlSaQvX5O+1dyjHmS4CF//ZwmSlsbntWoAt4Am+9xw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "duration": 262,
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
          "copyrightText": "© 2005 Echo Entertainment Pvt. Ltd.",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "455135",
                "name": "Shaan",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "685544",
                "name": "Subhomita",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/719/Tomaye-Bhalobeshe-Bengali-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomita-songs/gnA3DfCi9ac_"
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
                "id": "12106882",
                "name": "Anusua",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "457960",
                "name": "Indrajit",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/430/Agnipath-2005-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/indrajit-songs/zZPkC2HY7TE_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "455135",
                "name": "Shaan",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Shaan_002_20230605061056_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "685544",
                "name": "Subhomita",
                "role": "singer",
                "image": "https://c.saavncdn.com/719/Tomaye-Bhalobeshe-Bengali-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomita-songs/gnA3DfCi9ac_"
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
                "id": "12106882",
                "name": "Anusua",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "457960",
                "name": "Indrajit",
                "role": "singer",
                "image": "https://c.saavncdn.com/430/Agnipath-2005-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/indrajit-songs/zZPkC2HY7TE_"
              },
              {
                "id": "752807",
                "name": "Kaushik Chakraborty",
                "role": "lyricist",
                "image": "https://c.saavncdn.com/892/Baishnav-Jan-To-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kaushik-chakraborty-songs/5onpzEUYUXg_"
              },
              {
                "id": "898866",
                "name": "Kakoli",
                "role": "starring",
                "image": "https://c.saavncdn.com/584/Jab-Koi-Baat-Single-Hindi-2019-20190205041406-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kakoli-songs/dGdK9J3HzVs_"
              },
              {
                "id": "4999051",
                "name": "Sirin",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sirin-songs/H6EPoT0Boww_"
              },
              {
                "id": "855150",
                "name": "Shilajit",
                "role": "starring",
                "image": "https://c.saavncdn.com/038/Amra-Bengali-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilajit-songs/uvCPGIZ4hyw_"
              },
              {
                "id": "460250",
                "name": "Kanchan",
                "role": "starring",
                "image": "https://c.saavncdn.com/744/Tu-Meri-Rani-Banja-Hindi-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kanchan-songs/aY1OMEWgZnk_"
              },
              {
                "id": "787069",
                "name": "Milan",
                "role": "starring",
                "image": "https://c.saavncdn.com/493/Rewind-English-2017-20180608155233-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/milan-songs/TyLkRYqbZt4_"
              },
              {
                "id": "579296",
                "name": "Soma",
                "role": "starring",
                "image": "https://c.saavncdn.com/199/Amar-Moner-Koney-Bengali-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soma-songs/ZEzZRGQRTys_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "tZzjyO9I",
          "name": "Chaya Ghera",
          "subtitle": "Subhomitta - Tobu Aashte Hobe Phire",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/chaya-ghera/BDIRWw1-Dno",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2005,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Arijit Singh",
          "song": null,
          "albumId": "31739288",
          "album": "Tobu Aashte Hobe Phire",
          "label": "Echo Entertainment Pvt. Ltd.",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy63XSCgVtNVBmuUKmtS2CiUdW7huTUi1SqaQNp9mTDiQrMm+yegKKwBw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "duration": 309,
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
          "copyrightText": "© 2005 Echo Entertainment Pvt. Ltd.",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "12106884",
                "name": "Subhomitta",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomitta-songs/DZ2ZfULJ7Eo_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "12106884",
                "name": "Subhomitta",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomitta-songs/DZ2ZfULJ7Eo_"
              },
              {
                "id": "752807",
                "name": "Kaushik Chakraborty",
                "role": "lyricist",
                "image": "http://c.saavncdn.com/892/Baishnav-Jan-To-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kaushik-chakraborty-songs/5onpzEUYUXg_"
              },
              {
                "id": "898866",
                "name": "Kakoli",
                "role": "starring",
                "image": "http://c.saavncdn.com/584/Jab-Koi-Baat-Single-Hindi-2019-20190205041406-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kakoli-songs/dGdK9J3HzVs_"
              },
              {
                "id": "4999051",
                "name": "Sirin",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sirin-songs/H6EPoT0Boww_"
              },
              {
                "id": "855150",
                "name": "Shilajit",
                "role": "starring",
                "image": "http://c.saavncdn.com/038/Amra-Bengali-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilajit-songs/uvCPGIZ4hyw_"
              },
              {
                "id": "460250",
                "name": "Kanchan",
                "role": "starring",
                "image": "http://c.saavncdn.com/744/Tu-Meri-Rani-Banja-Hindi-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kanchan-songs/aY1OMEWgZnk_"
              },
              {
                "id": "787069",
                "name": "Milan",
                "role": "starring",
                "image": "http://c.saavncdn.com/493/Rewind-English-2017-20180608155233-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/milan-songs/TyLkRYqbZt4_"
              },
              {
                "id": "579296",
                "name": "Soma",
                "role": "starring",
                "image": "http://c.saavncdn.com/199/Amar-Moner-Koney-Bengali-2017-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soma-songs/ZEzZRGQRTys_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "coPfX34V",
          "name": "Amader Surya Merun",
          "subtitle": "Arijit Singh, Avik - Egaro The Eleven",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/amader-surya-merun/Ewc7VywDA2U",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2010,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Mayukh-Mainak",
          "song": null,
          "albumId": "1149027",
          "album": "Egaro The Eleven",
          "label": "Shemaroo Entertainment Audio",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyjKpVfftHLBIF+7HWoAiephR9KexsR0tZP3s///ClgD+ItQ9FQaQ3JBw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/egaro-the-eleven/jpUK6dUhOEE_",
          "duration": 234,
          "rights": {
            "code": "0",
            "cacheable": "true",
            "delete_cached_object": "false",
            "reason": ""
          },
          "cacheState": "false",
          "hasLyrics": true,
          "lyricsSnippet": "janmechhi maatha nie khelari parwana",
          "starred": false,
          "copyrightText": "©  2010 Shemaroo Entertainment Audio",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "547689",
                "name": "Avik",
                "role": "primary_artists",
                "image": "http://c.saavncdn.com/054/Ekphota-Brishti-2012-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/avik-songs/0Ygbf45uqK8_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "540706",
                "name": "Mayukh-Mainak",
                "role": "music",
                "image": "http://c.saavncdn.com/000/default_Saregama_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mayukh-mainak-songs/FUF1bvBxmrc_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singer",
                "image": "http://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "547689",
                "name": "Avik",
                "role": "singer",
                "image": "http://c.saavncdn.com/054/Ekphota-Brishti-2012-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/avik-songs/0Ygbf45uqK8_"
              },
              {
                "id": "684940",
                "name": "Soumonto Chowdhury",
                "role": "lyricist",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soumonto-chowdhury-songs/dHJJqf1nu5o_"
              },
              {
                "id": "684941",
                "name": "Heerok Das",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/heerok-das-songs/1lT3h8xopkk_"
              },
              {
                "id": "535837",
                "name": "Sankar Chakraborty",
                "role": "starring",
                "image": "http://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sankar-chakraborty-songs/PhSwZyNXl3I_"
              },
              {
                "id": "684942",
                "name": "Debparna",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/debparna-songs/1C1WxeqrxJ4_"
              },
              {
                "id": "684943",
                "name": "Ranadeep Bose",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ranadeep-bose-songs/VSsxVqoF5Gc_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": ""
        },
        {
          "id": "7UOkNbOl",
          "name": "E Shudhu Khela Noy",
          "subtitle": "Avik, Arijit Singh, Goutam - Egaro The Eleven",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/e-shudhu-khela-noy/Rz0kWjpSeF8",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-500x500.jpg"
            }
          ],
          "language": "bengali",
          "year": 2010,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Mayukh-Mainak",
          "song": null,
          "albumId": "1149027",
          "album": "Egaro The Eleven",
          "label": "Shemaroo Entertainment Audio",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyQqncYQ0jy5MJaMyorRF/InZTKrjH1l8LMbC/utbYnrIQwtAwGvGihxw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/egaro-the-eleven/jpUK6dUhOEE_",
          "duration": 212,
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
          "copyrightText": "©  2010 Shemaroo Entertainment Audio",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "547689",
                "name": "Avik",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/054/Ekphota-Brishti-2012-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/avik-songs/0Ygbf45uqK8_"
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
                "id": "455819",
                "name": "Goutam",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/557/Nandini-Tor-Chokhe-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/goutam-songs/QUMqgZ1jgRU_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "540706",
                "name": "Mayukh-Mainak",
                "role": "music",
                "image": "https://c.saavncdn.com/000/default_Saregama_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mayukh-mainak-songs/FUF1bvBxmrc_"
              },
              {
                "id": "547689",
                "name": "Avik",
                "role": "singer",
                "image": "https://c.saavncdn.com/054/Ekphota-Brishti-2012-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/avik-songs/0Ygbf45uqK8_"
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
                "id": "455819",
                "name": "Goutam",
                "role": "singer",
                "image": "https://c.saavncdn.com/557/Nandini-Tor-Chokhe-Bengali-2016-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/goutam-songs/QUMqgZ1jgRU_"
              },
              {
                "id": "684940",
                "name": "Soumonto Chowdhury",
                "role": "lyricist",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/soumonto-chowdhury-songs/dHJJqf1nu5o_"
              },
              {
                "id": "684941",
                "name": "Heerok Das",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/heerok-das-songs/1lT3h8xopkk_"
              },
              {
                "id": "535837",
                "name": "Sankar Chakraborty",
                "role": "starring",
                "image": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sankar-chakraborty-songs/PhSwZyNXl3I_"
              },
              {
                "id": "684942",
                "name": "Debparna",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/debparna-songs/1C1WxeqrxJ4_"
              },
              {
                "id": "684943",
                "name": "Ranadeep Bose",
                "role": "starring",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ranadeep-bose-songs/VSsxVqoF5Gc_"
              }
            ]
          },
          "releaseDate": "",
          "trillerAvailable": false,
          "lyricsId": null
        },
        {
          "id": "fcfGul86",
          "name": "Phir Mohabbat",
          "subtitle": "Mohammed Irfan, Arijit Singh, Saim Bhat - Murder 2",
          "type": "song",
          "url": "https://www.jiosaavn.com/song/phir-mohabbat/FgsNdgFcDwU",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-500x500.jpg"
            }
          ],
          "language": "hindi",
          "year": 2011,
          "playCount": 0,
          "explicitContent": false,
          "listCount": 0,
          "listType": "",
          "list": "",
          "music": "Mithoon, Mithoon",
          "song": null,
          "albumId": "1039687",
          "album": "Murder 2",
          "label": "",
          "origin": "song",
          "isDolbyContent": false,
          "320kbps": true,
          "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyKtFHGGMBppYkzBb0MZscBzvdmAM6mn00oM5gBYf48YBPms5G/nizuhw7tS9a8Gtq",
          "albumUrl": "https://www.jiosaavn.com/album/murder-2/evTOS8ealLo_",
          "duration": 329,
          "rights": {
            "code": "0",
            "cacheable": "true",
            "delete_cached_object": "false",
            "reason": ""
          },
          "cacheState": "false",
          "hasLyrics": true,
          "lyricsSnippet": "Dil. yaheen ruk ja zaraa",
          "starred": false,
          "copyrightText": "℗ 2011 Super Cassettes Industries Private Limited",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "743983",
                "name": "Mohammed Irfan",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mohammed-irfan-songs/yaf,eQp2Wcs_"
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
                "id": "480381",
                "name": "Saim Bhat",
                "role": "primary_artists",
                "image": "https://c.saavncdn.com/artists/Saim_Bhat_000_20210928052424_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saim-bhat-songs/bPLbYQZdZE4_"
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
                "id": "702592",
                "name": "Mithoon",
                "role": "music",
                "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              },
              {
                "id": "743983",
                "name": "Mohammed Irfan",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mohammed-irfan-songs/yaf,eQp2Wcs_"
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
                "id": "480381",
                "name": "Saim Bhat",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Saim_Bhat_000_20210928052424_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saim-bhat-songs/bPLbYQZdZE4_"
              },
              {
                "id": "669007",
                "name": "Sayeed Quadri",
                "role": "lyricist",
                "image": "https://c.saavncdn.com/artists/Sayeed_Quadri_001_20210616180918_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sayeed-quadri-songs/CkMFUTAeb,c_"
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
}
```

+++

|             **Query Parameter**             |            **Description**            |              **Required**               |
| :-----------------------------------------: | :-----------------------------------: | :-------------------------------------: |
|    [!badge variant="contrast" text="id"]    |               Artist ID               | [!badge variant="primary" text="True"]  |
|   [!badge variant="contrast" text="page"]   |              Page Number              | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="category"] | Song type. `alphabetical` or `latest` | [!badge variant="primary" text="False"] |
|   [!badge variant="contrast" text="sort"]   |   Song sort order. `asc` or `desc`    | [!badge variant="primary" text="False"] |

## Artist Albums by artist ID

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/artist/albums?id=459320
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/artist/albums?id=459320' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Artist albums fetched successfully!",
  "data": {
    "id": "459320",
    "name": "Arijit Singh",
    "image": [
      {
        "quality": "50x50",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_50x50.jpg"
      },
      {
        "quality": "150x150",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg"
      },
      {
        "quality": "500x500",
        "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_500x500.jpg"
      }
    ],
    "followerCount": 14584087,
    "type": "artist",
    "isVerified": true,
    "dominantLanguage": "hindi",
    "dominantType": "singer",
    "topAlbums": {
      "total": 881,
      "last_page": false,
      "albums": [
        {
          "id": "1034369",
          "name": "Kedi",
          "subtitle": "Sandeep Chowta",
          "type": "album",
          "language": "telugu",
          "playCount": 0,
          "explicit": false,
          "year": 2010,
          "url": "https://www.jiosaavn.com/album/kedi/,un5mYLHDao_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/369/Kedi-2010-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/369/Kedi-2010-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/369/Kedi-2010-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "458154",
                "name": "Sandeep Chowta",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandeep-chowta-songs/PACF9hBtu3M_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "458154",
                "name": "Sandeep Chowta",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandeep-chowta-songs/PACF9hBtu3M_"
              },
              {
                "id": "458154",
                "name": "Sandeep Chowta",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandeep-chowta-songs/PACF9hBtu3M_"
              },
              {
                "id": "455129",
                "name": "Sunidhi Chauhan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-songs/ZIyMwJzHJwI_"
              },
              {
                "id": "462297",
                "name": "Sonu Kakkar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sonu-kakkar-songs/XhCXrOfXgig_"
              },
              {
                "id": "455334",
                "name": "Rahul Nambiar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rahul-nambiar-songs/SFttMrTlVKo_"
              },
              {
                "id": "464932",
                "name": "Neha Kakkar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "476118",
                "name": "Saleem Shahida",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saleem-shahida-songs/nXFnTdxTNBU_"
              }
            ]
          },
          "songCount": 8,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "11837203",
          "name": "Love In Winter",
          "subtitle": "Various Artists",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 1999,
          "url": "https://www.jiosaavn.com/album/love-in-winter/ZIO5syEo90w_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/537/Love-In-Winter-Hindi-1999-20171121-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "455494",
                "name": "Various Artists",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/various-artists-songs/ztKx8IUBme8_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "1307410",
                "name": "Raamlaxman",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/raamlaxman-songs/BT-AKEkSAe0_"
              },
              {
                "id": "458866",
                "name": "Ravindra Jain",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ravindra-jain-songs/LieuwK4Ypnw_"
              },
              {
                "id": "456338",
                "name": "Anu Malik",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anu-malik-songs/mkfUaI-GTLM_"
              },
              {
                "id": "702592",
                "name": "Mithoon",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              },
              {
                "id": "455162",
                "name": "Hariharan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/hariharan-songs/Z2qyrGA65Yo_"
              },
              {
                "id": "455127",
                "name": "Udit Narayan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_"
              },
              {
                "id": "455130",
                "name": "Shreya Ghoshal",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
              },
              {
                "id": "455135",
                "name": "Shaan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "483224",
                "name": "K. S. Chithra",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/k.-s.-chithra-songs/doElLvdHCyk_"
              },
              {
                "id": "455782",
                "name": "KK",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kk-songs/K,5uNVM,qpM_"
              },
              {
                "id": "487919",
                "name": "Saurabh P. Srivastav",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saurabh-p.-srivastav-songs/827UWZBUa1o_"
              },
              {
                "id": "455120",
                "name": "Alka Yagnik",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "1984879",
                "name": "Chinmayi Sripada",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/chinmayi-sripada-songs/dUPdrrGYfPk_"
              }
            ]
          },
          "songCount": 11,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "31739288",
          "name": "Tobu Aashte Hobe Phire",
          "subtitle": "Arijit Singh",
          "type": "album",
          "language": "bengali",
          "playCount": 0,
          "explicit": false,
          "year": 2005,
          "url": "https://www.jiosaavn.com/album/tobu-aashte-hobe-phire/AvEFUWaUCys_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/601/Tobu-Aashte-Hobe-Phire-Bengali-2005-20211224194619-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "455135",
                "name": "Shaan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shaan-songs/4OWVZe3KEes_"
              },
              {
                "id": "685544",
                "name": "Subhomita",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomita-songs/gnA3DfCi9ac_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "12106882",
                "name": "Anusua",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anusua-songs/hdZRH2ujJ6s_"
              },
              {
                "id": "457960",
                "name": "Indrajit",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/indrajit-songs/zZPkC2HY7TE_"
              },
              {
                "id": "817440",
                "name": "Nipobithi",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nipobithi-songs/G,po7l7HZe0_"
              },
              {
                "id": "12106884",
                "name": "Subhomitta",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhomitta-songs/DZ2ZfULJ7Eo_"
              }
            ]
          },
          "songCount": 5,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "1149027",
          "name": "Egaro The Eleven",
          "subtitle": "Mayukh-Mainak",
          "type": "album",
          "language": "bengali",
          "playCount": 0,
          "explicit": false,
          "year": 2010,
          "url": "https://www.jiosaavn.com/album/egaro-the-eleven/jpUK6dUhOEE_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/974/Egaro-The-Eleven-Bengali-2010-20180809081226-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "540706",
                "name": "Mayukh-Mainak",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mayukh-mainak-songs/FUF1bvBxmrc_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "540706",
                "name": "Mayukh-Mainak",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mayukh-mainak-songs/FUF1bvBxmrc_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "547689",
                "name": "Avik",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/avik-songs/0Ygbf45uqK8_"
              },
              {
                "id": "455819",
                "name": "Goutam",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/goutam-songs/QUMqgZ1jgRU_"
              },
              {
                "id": "535837",
                "name": "Sankar Chakraborty",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sankar-chakraborty-songs/PhSwZyNXl3I_"
              }
            ]
          },
          "songCount": 3,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "1039687",
          "name": "Murder 2",
          "subtitle": "Harshit Saxena, Sangeet-Siddharth, Mithoon",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 2011,
          "url": "https://www.jiosaavn.com/album/murder-2/evTOS8ealLo_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/916/Murder-2-Hindi-2011-20221211193711-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "480382",
                "name": "Harshit Saxena",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/harshit-saxena-songs/ZgR4GYyADR8_"
              },
              {
                "id": "683706",
                "name": "Sangeet-Siddharth",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sangeet-siddharth-songs/JWsETS7JkVc_"
              },
              {
                "id": "702592",
                "name": "Mithoon",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "683706",
                "name": "Sangeet-Siddharth",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sangeet-siddharth-songs/JWsETS7JkVc_"
              },
              {
                "id": "702592",
                "name": "Mithoon",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              },
              {
                "id": "480382",
                "name": "Harshit Saxena",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/harshit-saxena-songs/ZgR4GYyADR8_"
              },
              {
                "id": "455129",
                "name": "Sunidhi Chauhan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sunidhi-chauhan-songs/ZIyMwJzHJwI_"
              },
              {
                "id": "743983",
                "name": "Mohammed Irfan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mohammed-irfan-songs/yaf,eQp2Wcs_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "480381",
                "name": "Saim Bhat",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saim-bhat-songs/bPLbYQZdZE4_"
              },
              {
                "id": "480382",
                "name": "Harshit Saxena",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/harshit-saxena-songs/ZgR4GYyADR8_"
              },
              {
                "id": "467875",
                "name": "Kshitij Tarey",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kshitij-tarey-songs/vZ6XOMs-US4_"
              },
              {
                "id": "702592",
                "name": "Mithoon",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              },
              {
                "id": "464818",
                "name": "Sangeet Haldipur",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sangeet-haldipur-songs/pQp86,fLk,A_"
              },
              {
                "id": "480383",
                "name": "Roshni Baptist",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/roshni-baptist-songs/GIdPERxhjb0_"
              }
            ]
          },
          "songCount": 8,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "14791159",
          "name": "Iti Mrinalini",
          "subtitle": "Sunny, Arijit Singh",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 2011,
          "url": "https://www.jiosaavn.com/album/iti-mrinalini/kcGmUwdkCiw_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/625/Iti-Mrinalini-Hindi-2011-20210708175411-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/625/Iti-Mrinalini-Hindi-2011-20210708175411-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/625/Iti-Mrinalini-Hindi-2011-20210708175411-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "469536",
                "name": "Sunny",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sunny-songs/VGWMoJvGjXw_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "468220",
                "name": "Debojyoti Mishra",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/debojyoti-mishra-songs/WXKfEcAw72g_"
              },
              {
                "id": "469536",
                "name": "Sunny",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sunny-songs/VGWMoJvGjXw_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "455130",
                "name": "Shreya Ghoshal",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
              },
              {
                "id": "464595",
                "name": "Aparna Sen",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/aparna-sen-songs/xwA2bWrmyvg_"
              }
            ]
          },
          "songCount": 5,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "1043066",
          "name": "Players",
          "subtitle": "Pritam",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 2011,
          "url": "https://www.jiosaavn.com/album/players/6Y0yMSuyZds_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/122/Players-Hindi-2011-20221212012142-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/122/Players-Hindi-2011-20221212012142-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/122/Players-Hindi-2011-20221212012142-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "456323",
                "name": "Pritam",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "456323",
                "name": "Pritam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
              },
              {
                "id": "455130",
                "name": "Shreya Ghoshal",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
              },
              {
                "id": "455124",
                "name": "Mohit Chauhan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mohit-chauhan-songs/YNIh2poJKsY_"
              },
              {
                "id": "459576",
                "name": "Nikhil D&#039;souza",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nikhil-dsouza-songs/-dscRpTcQBo_"
              },
              {
                "id": "472749",
                "name": "Priyani Vani",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/priyani-vani-songs/tZeNnzUPLCI_"
              },
              {
                "id": "458921",
                "name": "Ritu Pathak",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ritu-pathak-songs/6MNcjT0eH78_"
              },
              {
                "id": "461363",
                "name": "Abhishek Bachchan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/abhishek-bachchan-songs/g4jK4dRXJfM_"
              },
              {
                "id": "455694",
                "name": "Neeraj Shridhar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/neeraj-shridhar-songs/-h0RbDzaCqA_"
              },
              {
                "id": "463014",
                "name": "Mauli Dave",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mauli-dave-songs/UIMY5VWmFRI_"
              },
              {
                "id": "482886",
                "name": "Siddharth Basrur",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/siddharth-basrur-songs/uDwDFPrgbjI_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "461476",
                "name": "Bob",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/bob-songs/38hI2oV2VI4_"
              },
              {
                "id": "482887",
                "name": "Yashita Yashpal",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/yashita-yashpal-songs/OgKc0DCfvzs_"
              },
              {
                "id": "459149",
                "name": "Shruti Pathak",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shruti-pathak-songs/9-O2udXMr5Q_"
              },
              {
                "id": "480295",
                "name": "Earl Edgar (URL)",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/earl-edgar-url-songs/sRzlK6asDvs_"
              }
            ]
          },
          "songCount": 21,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "1018603",
          "name": "Agent Vinod",
          "subtitle": "Pritam",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 2012,
          "url": "https://www.jiosaavn.com/album/agent-vinod/tkfii,Hayx0_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/603/Agent-Vinod-2012-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/603/Agent-Vinod-2012-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/603/Agent-Vinod-2012-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "456323",
                "name": "Pritam",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "456323",
                "name": "Pritam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              },
              {
                "id": "456206",
                "name": "Aditi Singh Sharma",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/aditi-singh-sharma-songs/PA8-JR,fTU4_"
              },
              {
                "id": "456323",
                "name": "Pritam",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
              },
              {
                "id": "459664",
                "name": "Hamsika Iyer",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/hamsika-iyer-songs/,s5T5MPb9Z4_"
              },
              {
                "id": "459153",
                "name": "Joi Barua",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/joi-barua-songs/eRoh5nwsHeg_"
              },
              {
                "id": "1204343",
                "name": "Joi",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/joi-songs/ybujDrGb4W8_"
              },
              {
                "id": "455694",
                "name": "Neeraj Shridhar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/neeraj-shridhar-songs/-h0RbDzaCqA_"
              },
              {
                "id": "546078",
                "name": "Shefali Alvares",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shefali-alvares-songs/3SkYUbVyWdE_"
              },
              {
                "id": "459318",
                "name": "Barbie Amod",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/barbie-amod-songs/cawlqPswCs0_"
              },
              {
                "id": "459585",
                "name": "Nakash Aziz",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nakash-aziz-songs/,wWkOOUle4o_"
              },
              {
                "id": "459085",
                "name": "Mika Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mika-singh-songs/o-4rXBimwaE_"
              },
              {
                "id": "458681",
                "name": "Amitabh Bhattacharya",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
              },
              {
                "id": "459315",
                "name": "Nakash",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nakash-songs/i2Bwvdmjx6w_"
              },
              {
                "id": "719626",
                "name": "Javed Jaffrey",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/javed-jaffrey-songs/KqAOU7ml8E4_"
              },
              {
                "id": "455130",
                "name": "Shreya Ghoshal",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shreya-ghoshal-songs/lIHlwHaxTZ0_"
              },
              {
                "id": "465308",
                "name": "Nandini Srikar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nandini-srikar-songs/OhKMp3wdQyo_"
              },
              {
                "id": "481520",
                "name": "Altamash Faridi",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/altamash-faridi-songs/OmA4hfy6YiM_"
              },
              {
                "id": "459326",
                "name": "Muazzam",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/muazzam-songs/sTotgm,pZ8Y_"
              },
              {
                "id": "459327",
                "name": "Rizwan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rizwan-songs/0eXs1ps,DEA_"
              },
              {
                "id": "909878",
                "name": "Rizwan-Muazzam",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rizwan-muazzam-songs/xApfz-M4alU_"
              },
              {
                "id": "459325",
                "name": "Shadab Faridi",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shadab-faridi-songs/IcPgWuDD3N4_"
              },
              {
                "id": "459324",
                "name": "Shabab Sabri",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shabab-sabri-songs/ZG6gtCp2hi8_"
              },
              {
                "id": "464477",
                "name": "Malini Awasthi",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/malini-awasthi-songs/W8-RyqSNNi4_"
              }
            ]
          },
          "songCount": 11,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "1070167",
          "name": "Shanghai",
          "subtitle": "Vishal &amp; Shekhar",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 2012,
          "url": "https://www.jiosaavn.com/album/shanghai/hVlg998vFiI_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/211/Shanghai-Hindi-2012-20221212054037-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/211/Shanghai-Hindi-2012-20221212054037-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/211/Shanghai-Hindi-2012-20221212054037-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "459880",
                "name": "Vishal &amp; Shekhar",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "459880",
                "name": "Vishal &amp; Shekhar",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vishal-shekhar-songs/8swM0KpGBus_"
              },
              {
                "id": "456144",
                "name": "Raja Hasan",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/raja-hasan-songs/fKnNcUKL5wY_"
              },
              {
                "id": "467782",
                "name": "Keerthi Sagathia",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/keerthi-sagathia-songs/gYlP8CO25Tw_"
              },
              {
                "id": "455669",
                "name": "Vishal Dadlani",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vishal-dadlani-songs/6o-AMde03I8_"
              },
              {
                "id": "459883",
                "name": "Shekhar Ravjiani",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shekhar-ravjiani-songs/uJWQLrQp8Og_"
              },
              {
                "id": "531900",
                "name": "Srivatsa Krishna",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/srivatsa-krishna-songs/E6J90AY4Bi4_"
              },
              {
                "id": "456392",
                "name": "Richa Sharma",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/richa-sharma-songs/VoLWFjaqUmg_"
              },
              {
                "id": "465308",
                "name": "Nandini Srikar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nandini-srikar-songs/OhKMp3wdQyo_"
              },
              {
                "id": "459320",
                "name": "Arijit Singh",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
              }
            ]
          },
          "songCount": 8,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "34939814",
          "name": "Sudhu Bhalobese Gelam",
          "subtitle": "Ranajit Basu Ray",
          "type": "album",
          "language": "bengali",
          "playCount": 0,
          "explicit": false,
          "year": 2012,
          "url": "https://www.jiosaavn.com/album/sudhu-bhalobese-gelam/dFYYJmKErnU_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/228/Sudhu-Bhalobese-Gelam-Bengali-2012-20220512175601-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/228/Sudhu-Bhalobese-Gelam-Bengali-2012-20220512175601-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/228/Sudhu-Bhalobese-Gelam-Bengali-2012-20220512175601-500x500.jpg"
            }
          ],
          "releaseDate": null,
          "artistMap": {
            "primaryArtists": [
              {
                "id": "13074883",
                "name": "Ranajit Basu Ray",
                "role": "primary_artists",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ranajit-basu-ray-songs/RqGsS6xJ5vM_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "13074883",
                "name": "Ranajit Basu Ray",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ranajit-basu-ray-songs/RqGsS6xJ5vM_"
              },
              {
                "id": "13074885",
                "name": "Mohua Goswami",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mohua-goswami-songs/PoNuINABO48_"
              },
              {
                "id": "13074886",
                "name": "Subhash Mrida",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/subhash-mrida-songs/49GptD6UrTg_"
              },
              {
                "id": "455142",
                "name": "Kumar Sanu",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/kumar-sanu-songs/fe0z9ZAFgGE_"
              },
              {
                "id": "13074882",
                "name": "Poulami Ghosh Dastidar",
                "role": "singers",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/poulami-ghosh-dastidar-songs/5wVPnCcbjj0_"
              }
            ]
          },
          "songCount": 5,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        }
      ]
    }
  }
}
```

+++

|             **Query Parameter**             |            **Description**             |              **Required**               |
| :-----------------------------------------: | :------------------------------------: | :-------------------------------------: |
|    [!badge variant="contrast" text="id"]    |               Artist ID                | [!badge variant="primary" text="True"]  |
|   [!badge variant="contrast" text="page"]   |              Page Number               | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="category"] | Album type. `alphabetical` or `latest` | [!badge variant="primary" text="False"] |
|   [!badge variant="contrast" text="sort"]   |   Album sort order. `asc` or `desc`    | [!badge variant="primary" text="False"] |

## Artist Top / Recommended Songs by artist ID and song ID

!!!
Note: Song ID is required to get the recommendations based on the song.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/artist/recommendations?songId=_rJmbKSP&artistId=459320
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/artist/recommendations?songId=_rJmbKSP&artistId=459320' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Recommended artists songs fetched successfully!",
  "data": [
    {
      "id": "bITJbD1j",
      "name": "I Need You",
      "subtitle": "Semwal, Arijit Singh - I Need You",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/i-need-you/EiE-exZ0Blk",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/836/I-Need-You-English-2022-20220809131953-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/836/I-Need-You-English-2022-20220809131953-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/836/I-Need-You-English-2022-20220809131953-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2022,
      "playCount": 1801,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "",
      "song": null,
      "albumId": "37186319",
      "album": "I Need You",
      "label": "The Music Room",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyt/bpRjC7ZjeTgZkPAf5csYCVGpVG/rTkTE5/45Ndne8R9AKpodTeZxw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/i-need-you/dvHLXB7Q8aQ_",
      "duration": 154,
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
      "copyrightText": "℗ 2022 Shubham Semwal",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "7790319",
            "name": "Semwal",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Semwal_001_20230118091137_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/semwal-songs/UjB9sfzgx0k_"
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
            "id": "7790319",
            "name": "Semwal",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Semwal_001_20230118091137_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/semwal-songs/UjB9sfzgx0k_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2022-08-26",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "gSDFUxeU",
      "name": "Neki Ki Raah (Slowed &amp; Reverb)",
      "subtitle": "Rabiul Rhmn - Neki Ki Raah (Slowed &amp; Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/neki-ki-raah-slowed-reverb/FzsvdyFIUmY",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/263/Neki-Ki-Raah-Slowed-Reverb-English-2023-20230804214125-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/263/Neki-Ki-Raah-Slowed-Reverb-English-2023-20230804214125-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/263/Neki-Ki-Raah-Slowed-Reverb-English-2023-20230804214125-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 203,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh, Mithoon",
      "song": null,
      "albumId": "47277632",
      "album": "Neki Ki Raah (Slowed &amp; Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyVTK/14Qgu6jl8Kq/Wd0eeUWgyu2frTqYL4jwOu8GPCOMKI8d7IoX8Rw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/neki-ki-raah-slowed-reverb/3U,LpsgWsq8_",
      "duration": 195,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "702592",
            "name": "Mithoon",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "702592",
            "name": "Mithoon",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Mithoon_002_20200908073735_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
          }
        ]
      },
      "releaseDate": "2023-08-07",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "EwRPJUpy",
      "name": "Chaleya (Slowed and Reverb)",
      "subtitle": "Røbî, Rabiul Rhmn - Chaleya [Jawan] (Slowed and Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/chaleya-slowed-and-reverb/NR85YT5lR0o",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/908/Chaleya-Jawan-Slowed-and-Reverb-English-2023-20230816045623-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/908/Chaleya-Jawan-Slowed-and-Reverb-English-2023-20230816045623-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/908/Chaleya-Jawan-Slowed-and-Reverb-English-2023-20230816045623-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 11,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Rabiul Rhmn, Arijit Singh",
      "song": null,
      "albumId": "47583682",
      "album": "Chaleya [Jawan] (Slowed and Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyCj51k6Fm6iEdmFOSq8tZ/F+s8mfLsEM0j1ivGfnkyr7AYSSub2ehQhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/chaleya-jawan-slowed-and-reverb/2R7qO7MZ9sE_",
      "duration": 169,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "17084702",
            "name": "Røbî",
            "role": "primary_artists",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/robi-songs/b76ag6ASlZI_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
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
            "id": "17084702",
            "name": "Røbî",
            "role": "singer",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/robi-songs/b76ag6ASlZI_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-15",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "eI5DajU7",
      "name": "HUMDARD (Lofi)",
      "subtitle": "Ravi Raj - HUMDARD (Lofi)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/humdard-lofi/FSFedRVaYgQ",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/394/HUMDARD-Lofi-English-2023-20230620190837-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/394/HUMDARD-Lofi-English-2023-20230620190837-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/394/HUMDARD-Lofi-English-2023-20230620190837-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 203,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "46070322",
      "album": "HUMDARD (Lofi)",
      "label": "Independent",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy3zvz/6+nMsRihPyUuN5KTfFSxdEmyHYrlCRQbOD2aw18ixxnFG3QABw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/humdard-lofi/PhUFqaN38AQ_",
      "duration": 148,
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
      "copyrightText": "℗ 2023 Ravi Raj",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "482834",
            "name": "Ravi Raj",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Ravi_Raj_000_20210518091353_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/ravi-raj-songs/bzXyneWI1dA_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "482834",
            "name": "Ravi Raj",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Ravi_Raj_000_20210518091353_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/ravi-raj-songs/bzXyneWI1dA_"
          }
        ]
      },
      "releaseDate": "2023-06-27",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "a5kj8V3s",
      "name": "Chal Wahan Jaate Hai (Slowed &amp; Reverb)",
      "subtitle": "Rabiul Rhmn - Main Dhoondne (slowed &amp; reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/chal-wahan-jaate-hai-slowed-reverb/EV0AW0xmBEA",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/225/Main-Dhoondne-slowed-reverb-English-2023-20230804211019-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/225/Main-Dhoondne-slowed-reverb-English-2023-20230804211019-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/225/Main-Dhoondne-slowed-reverb-English-2023-20230804211019-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 84,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "47276932",
      "album": "Main Dhoondne (slowed &amp; reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyiCL1fYuuI6kxzZmxdF+K0fIDlxefW6Fv69z8R1/l0WlGru1BX2kDGRw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/main-dhoondne-slowed-reverb/zZeu1IZDEK4_",
      "duration": 235,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-07",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "AS66ORBs",
      "name": "Zaalima - (Slowed + Reverb)",
      "subtitle": "Rabiul Rhmn, Arijit Singh - Zaalima - (Slowed + Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/zaalima-slowed-%2b-reverb/MTtdBztidUA",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/419/Zaalima-Slowed-Reverb-English-2023-20230802052622-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/419/Zaalima-Slowed-Reverb-English-2023-20230802052622-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/419/Zaalima-Slowed-Reverb-English-2023-20230802052622-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 669,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "47196405",
      "album": "Zaalima - (Slowed + Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyHHaQ94tg8P0j84n0kQx/3XDDJUG7x0eD8uWP8ZuDj7VGW9/QZvnJGhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/zaalima-slowed-%2b-reverb/EEOKr6IpW7M_",
      "duration": 169,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
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
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
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
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-01",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "LE8Dwwrr",
      "name": "Mast Magan (Slowed &amp; Reverb)",
      "subtitle": "Rabiul Rhmn - Mast Magan (Slowed &amp; Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/mast-magan-slowed-reverb/PC1TdQNHRUE",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/249/Mast-Magan-Slowed-Reverb-English-2023-20230804214157-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/249/Mast-Magan-Slowed-Reverb-English-2023-20230804214157-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/249/Mast-Magan-Slowed-Reverb-English-2023-20230804214157-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 124,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "47277630",
      "album": "Mast Magan (Slowed &amp; Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy54FmfJPX+pXtmqFsxdCPzOwYompOZlGoYbQJR7eeW16oysRzpI89hhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/mast-magan-slowed-reverb/65Ecg9nrFHQ_",
      "duration": 123,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-07",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "3uBI60P4",
      "name": "Samjhawan - (Slowed + Reverb)",
      "subtitle": "Rabiul Rhmn - Samjhawan - (Slowed + Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/samjhawan-slowed-%2b-reverb/Qx0peEIAZwc",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/426/Samjhawan-Slowed-Reverb-English-2023-20230802052554-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/426/Samjhawan-Slowed-Reverb-English-2023-20230802052554-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/426/Samjhawan-Slowed-Reverb-English-2023-20230802052554-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 181,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "47196406",
      "album": "Samjhawan - (Slowed + Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDybDKM/ffa+ix26Xo93tNgtXxIxum54xLXWwIsh2yZXxh7EX8E9zkaaRw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/samjhawan-slowed-%2b-reverb/xu7BLT-iN7Q_",
      "duration": 152,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-01",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "J9kOubms",
      "name": "Shayad Lo-Fi",
      "subtitle": "Praveen Pratap Singh, Arijit Singh - Shayad Lo-Fi",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/shayad-lo-fi/OlEAfgFSWkA",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/998/Shayad-Lo-Fi-English-2022-20220610131437-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/998/Shayad-Lo-Fi-English-2022-20220610131437-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/998/Shayad-Lo-Fi-English-2022-20220610131437-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2022,
      "playCount": 364,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Praveen Pratap Singh",
      "song": null,
      "albumId": "35635083",
      "album": "Shayad Lo-Fi",
      "label": "Factdarshan Times",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyHxccA5Z98YDPRjNWzdHXPWa9/e8yrcyAIarkeNMoqBkQQ1+hQOlRKBw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/shayad-lo-fi/Trs0njTYK3Q_",
      "duration": 311,
      "rights": {
        "code": "1",
        "cacheable": "false",
        "delete_cached_object": "true",
        "reason": "Unavailable"
      },
      "cacheState": "false",
      "hasLyrics": false,
      "lyricsSnippet": "",
      "starred": false,
      "copyrightText": "℗ 2022 Factdarshan Times",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "13317192",
            "name": "Praveen Pratap Singh",
            "role": "primary_artists",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/praveen-pratap-singh-songs/1fTrNocf-qw_"
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
            "id": "13317192",
            "name": "Praveen Pratap Singh",
            "role": "music",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/praveen-pratap-singh-songs/1fTrNocf-qw_"
          },
          {
            "id": "13317192",
            "name": "Praveen Pratap Singh",
            "role": "singer",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/praveen-pratap-singh-songs/1fTrNocf-qw_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2022-04-09",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "JLW1uN-L",
      "name": "Shaami Mildi",
      "subtitle": "Roop Ghuman, Vibhuti Joshi, Arijit Singh - Shaami Mildi",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/shaami-mildi/OiQ8AAF,Gn8",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/911/Shaami-Mildi-English-2020-20200904094744-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/911/Shaami-Mildi-English-2020-20200904094744-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/911/Shaami-Mildi-English-2020-20200904094744-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2020,
      "playCount": 4655,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Rupinder Singh, Vibhuti Joshi, Arijit Singh",
      "song": null,
      "albumId": "22302822",
      "album": "Shaami Mildi",
      "label": "Roop Ghuman",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy0x/80CIhnOK1RP5c+57fjtHk+DoNI1nL0FIZ7IdI8mvEiyb1TMDOVBw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/shaami-mildi/fJbmRXuGLng_",
      "duration": 210,
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
      "copyrightText": "© 2020 Roop Ghuman",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "5857160",
            "name": "Roop Ghuman",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Roop_Ghuman_000_20210629053314_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/roop-ghuman-songs/mB8VLCH2mys_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
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
            "id": "2927841",
            "name": "Rupinder Singh",
            "role": "music",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rupinder-singh-songs/VhSRgONelqQ_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
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
            "id": "5857160",
            "name": "Roop Ghuman",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Roop_Ghuman_000_20210629053314_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/roop-ghuman-songs/mB8VLCH2mys_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2020-09-12",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "DZv7esWz",
      "name": "Arijit Singh - Naina [Slowed+ Reverb]",
      "subtitle": "Rabiul Rhmn, Arijit Singh - Arijit Singh - Naina [Slowed+ Reverb]",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/arijit-singh-naina-slowed%2b-reverb/NDIdBhFDYEk",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/915/Arijit-Singh-Naina-Slowed-Reverb-English-2023-20230725052654-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/915/Arijit-Singh-Naina-Slowed-Reverb-English-2023-20230725052654-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/915/Arijit-Singh-Naina-Slowed-Reverb-English-2023-20230725052654-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 1316,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Rabiul Rhmn",
      "song": null,
      "albumId": "46977887",
      "album": "Arijit Singh - Naina [Slowed+ Reverb]",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyIzjEWuCtREmZL5N9DlsEnJePeXM0/X7sBQK65Iin8ZGOxUjPFLGKAhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/arijit-singh-naina-slowed%2b-reverb/GbwvHIe8XFw_",
      "duration": 176,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
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
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
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
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ]
      },
      "releaseDate": "2023-07-25",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "ddOAWDSC",
      "name": "Bollywood Mashup",
      "subtitle": "Raghav Sehgal - Bollywood Mashup",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/bollywood-mashup/FAwkcCN0ZHA",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/014/Bollywood-Mashup-English-2021-20230122124642-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/014/Bollywood-Mashup-English-2021-20230122124642-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/014/Bollywood-Mashup-English-2021-20230122124642-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2021,
      "playCount": 203,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Raghav Sehgal, Arijit Singh, Pritam, Jagjit Singh, Sameer, Amitabh Bhattacharya, Faaiz Anwar",
      "song": null,
      "albumId": "41903053",
      "album": "Bollywood Mashup",
      "label": "Raghav Sehgal",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyGdrg7Cu0z9vUS4NWWNPfi7Ud3qJpOv0NudLw+UkcplMKeAbyWTb02xw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/bollywood-mashup/yFPqC12P-zk_",
      "duration": 166,
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
      "copyrightText": "© 2021 Raghav Sehgal",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "8619700",
            "name": "Raghav Sehgal",
            "role": "primary_artists",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/raghav-sehgal-songs/ZeFwH,avACw_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "8619700",
            "name": "Raghav Sehgal",
            "role": "music",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/raghav-sehgal-songs/ZeFwH,avACw_"
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
            "id": "456323",
            "name": "Pritam",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Pritam_Chakraborty-20170711073326_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/pritam-songs/OaFg9HPZgq8_"
          },
          {
            "id": "455948",
            "name": "Jagjit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Jagjit_Singh_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/jagjit-singh-songs/EMskk1sVOtM_"
          },
          {
            "id": "455415",
            "name": "Sameer",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Sameer-20170714064856_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/sameer-songs/zbXOIZIhW-8_"
          },
          {
            "id": "458681",
            "name": "Amitabh Bhattacharya",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Amitabh_Bhattacharya_000_20220916184017_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/amitabh-bhattacharya-songs/hsNRL6ZmJmo_"
          },
          {
            "id": "468469",
            "name": "Faaiz Anwar",
            "role": "music",
            "image": "https://c.saavncdn.com/437/Dil-Ke-Sau-Tukde-Har-Tukde-Pe-Tera-Naam-Hai-2013-150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/faaiz-anwar-songs/r7yeO0G3Mps_"
          },
          {
            "id": "8619700",
            "name": "Raghav Sehgal",
            "role": "singer",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/raghav-sehgal-songs/ZeFwH,avACw_"
          }
        ]
      },
      "releaseDate": "2021-02-03",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "F4wOecK8",
      "name": "Humdard (Slowed &amp; Reverb)",
      "subtitle": "Rabiul Rhmn - Humdard (Slowed &amp; Reverb)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/humdard-slowed-reverb/NlwcfhFTfAs",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/256/Humdard-Slowed-Reverb-English-2023-20230804214228-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/256/Humdard-Slowed-Reverb-English-2023-20230804214228-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/256/Humdard-Slowed-Reverb-English-2023-20230804214228-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 101,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "47277631",
      "album": "Humdard (Slowed &amp; Reverb)",
      "label": "Rabiul Records",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy5PlH99yt7P7zT98M/M0BFxosJTJyCxvxU9is5hIcTaEyDDk2wBYgMhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/humdard-slowed-reverb/,H8F88xNqNI_",
      "duration": 166,
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
      "copyrightText": "℗ 2023 Rabiul Records",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          }
        ],
        "featuredArtists": [],
        "artists": [
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          },
          {
            "id": "14829828",
            "name": "Rabiul Rhmn",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Rabiul_Rhmn_000_20230130081647_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rabiul-rhmn-songs/4CGhDGjVOEI_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-08-07",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "gvZMEhjA",
      "name": "kill the show",
      "subtitle": "Arijit Singh - kill the show",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/kill-the-show/Fx4xfDFYXXI",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/371/kill-the-show-English-2023-20230310061839-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/371/kill-the-show-English-2023-20230310061839-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/371/kill-the-show-English-2023-20230310061839-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2023,
      "playCount": 831,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Arijit Singh",
      "song": null,
      "albumId": "43383922",
      "album": "kill the show",
      "label": "Arijit entertainment",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDy+vXP4MuO+3DkR/2qMARMoJYydl7s19HexMz+yUg6+SvAbGzQDf0MBhw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/kill-the-show/KmeNYfGKNnQ_",
      "duration": 109,
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
      "copyrightText": "℗ 2023 Arijit Entertainment india , Exclusive Licensed Under Arena Music",
      "artistMap": {
        "primaryArtists": [
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
            "id": "459320",
            "name": "Arijit Singh",
            "role": "music",
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
            "id": "459320",
            "name": "Arijit Singh",
            "role": "lyricist",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2023-03-23",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "AMSeR6SO",
      "name": "Dua Karo - Reprise",
      "subtitle": "Shivesh Dwivedi, Arijit Singh - Dua Karo - Reprise",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/dua-karo-reprise/MSU4VCYGZHw",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/285/Dua-Karo-Reprise-English-2020-20200501014729-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/285/Dua-Karo-Reprise-English-2020-20200501014729-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/285/Dua-Karo-Reprise-English-2020-20200501014729-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2020,
      "playCount": 2528,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "",
      "song": null,
      "albumId": "20145686",
      "album": "Dua Karo - Reprise",
      "label": "",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyPeyEZPYv9AkEIzMfBAYKguTI4tlLrIO95zShN3gDZ4ZRscLIKLQCVRw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/dua-karo-reprise/2nQy3Sc28pQ_",
      "duration": 156,
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
      "copyrightText": "℗ 2020 T-Series",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "7876106",
            "name": "Shivesh Dwivedi",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Shivesh_Dwivedi_001_20210928060941_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/shivesh-dwivedi-songs/IVeNU1fdugo_"
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
            "id": "7876106",
            "name": "Shivesh Dwivedi",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Shivesh_Dwivedi_001_20210928060941_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/shivesh-dwivedi-songs/IVeNU1fdugo_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2020-02-04",
      "trillerAvailable": false,
      "lyricsId": null
    },
    {
      "id": "gU-yuSTf",
      "name": "Saare Chann Te Taare (Revisited)",
      "subtitle": "Roop Ghuman, Vibhuti Joshi, Arijit Singh - Saare Chann Te Taare (Revisited)",
      "type": "song",
      "url": "https://www.jiosaavn.com/song/saare-chann-te-taare-revisited/Fz1GSAFjY1U",
      "image": [
        {
          "quality": "50x50",
          "link": "https://c.saavncdn.com/841/Saare-Chann-Te-Taare-Revisited--English-2021-20210422202357-50x50.jpg"
        },
        {
          "quality": "150x150",
          "link": "https://c.saavncdn.com/841/Saare-Chann-Te-Taare-Revisited--English-2021-20210422202357-150x150.jpg"
        },
        {
          "quality": "500x500",
          "link": "https://c.saavncdn.com/841/Saare-Chann-Te-Taare-Revisited--English-2021-20210422202357-500x500.jpg"
        }
      ],
      "language": "english",
      "year": 2021,
      "playCount": 4432,
      "explicitContent": false,
      "listCount": 0,
      "listType": "",
      "list": "",
      "music": "Rupinder Singh, Vibhuti Joshi, Arijit Singh",
      "song": null,
      "albumId": "26736337",
      "album": "Saare Chann Te Taare (Revisited)",
      "label": "Roop Ghuman",
      "origin": "none",
      "isDolbyContent": false,
      "320kbps": true,
      "downloadUrl": "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDysVlyeiF+N3A8pfOMsXpUmyk1pg6IyfYnqe7A+qcnE8jHnKRJlvGNOxw7tS9a8Gtq",
      "albumUrl": "https://www.jiosaavn.com/album/saare-chann-te-taare-revisited/IJYJMuYzWVU_",
      "duration": 212,
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
      "copyrightText": "© 2021 Roop Ghuman",
      "artistMap": {
        "primaryArtists": [
          {
            "id": "5857160",
            "name": "Roop Ghuman",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Roop_Ghuman_000_20210629053314_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/roop-ghuman-songs/mB8VLCH2mys_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "primary_artists",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
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
            "id": "2927841",
            "name": "Rupinder Singh",
            "role": "music",
            "image": "",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/rupinder-singh-songs/VhSRgONelqQ_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "music",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
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
            "id": "5857160",
            "name": "Roop Ghuman",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Roop_Ghuman_000_20210629053314_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/roop-ghuman-songs/mB8VLCH2mys_"
          },
          {
            "id": "8606340",
            "name": "Vibhuti Joshi",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Vibhuti_Joshi_000_20230212184902_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/vibhuti-joshi-songs/7o18UfG4HaE_"
          },
          {
            "id": "459320",
            "name": "Arijit Singh",
            "role": "singer",
            "image": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg",
            "type": "artist",
            "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_"
          }
        ]
      },
      "releaseDate": "2021-05-05",
      "trillerAvailable": false,
      "lyricsId": null
    }
  ]
}
```

+++

|             **Query Parameter**             | **Description** |              **Required**               |
| :-----------------------------------------: | :-------------: | :-------------------------------------: |
|    [!badge variant="contrast" text="id"]    |     Song ID     | [!badge variant="primary" text="True"]  |
|    [!badge variant="contrast" text="id"]    |    Artist ID    | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="language"] | Songs language  | [!badge variant="primary" text="False"] |
