-- 小组
create table `dong`.groups (
`access`: 0,
`activity`: 0
`answer`: ""
`answer_hash`: ""
`classifications`: "101;104;106;3;45;73"
`code`: "1016957841"
`copy_extra`: ""
`copy_state`: 1
`created`: "2023-01-05T23:21:49.936784+08:00"
`creator`: "custerfun"
`default_order_by`: 0
`description`: "小组简介"
`end_day`: ""
`end_time`:null
`ended`: null
`id`: 13961
`img`: "http://www.aboboo.com/media/groups/image/2023/01/05/custerfun/resized/1f4d8cf5_320.png"
`img_is_default`: false
`is_alive`: true
`itype`: 0
`kinds`:""
`modified`: "2023-03-30T15:49:48.378360+08:00"
`name`: "小组名称"
`owner`: "custerfun"
`people`: 0
`people_old`: 0
`question`: ""
`retention`: 0
`slogan`: "小组口号"
`start_day`: ""
`start_time`: null
`timezone`:"PRC"
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci comment '项目表';

-- 小组成员
create table `dong`.groups_members (
    "id": 4037112,
    "message": "",
    "position": 1,
    "about": null,
    "username": "double188",
    "avatar": "http://download.aboboo.com/static/avatar/green/80.png"
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci comment '项目表';

-- 小组课件
create table `dong`.groups_courseware (
      "id": 186912,
      "gres_creator_id": 2366392,
      "gres_id": 244364,
      "gres_group_id": 13961,
      "gres_size": 3568068,
      "gres_created": "2023-01-10 08:32:05+0800",
      "gres_mimetype": "application/q99",
      "gres_download_times": 89,
      "gres_sort_order": "",
      "gres_download_url": "/api/0.1/groups/down/1016957841/244364/",
      "gres_group_code": "1016957841",
      "lang": "英语",
      "name": "PASSAGE 1: Success in Life",
      "class1": "雅思",
      "class2": "口语",
      "created": "2023-01-27 23:11:22+0800",
      "group_id": 13961,
      "modified": "2023-01-28 00:09:10+0800",
      "union_hash": "E534B6F3CC9A9F814F6850104D6C6680",
      "lessonnum": 0,
      "lesson_number": 0,
      "revision_number": 1,
      "itype": 1,
      "is_private": false,
      "iprivilege": 2,
      "discount": 0,
      "filemd5": "AB037E9520165E01252EB3775E138DD6",
      "simpleDesc": "",
      "simple_desc": "",
      "download_url": "",
      "creator_id": 2366392,
      "file_content_size": 3580539,
      "file_size": 3580539,
      "include_content": 1,
      "is_published": true,
      "sents": 28,
      "author": "Custer",
      "is_trial": false,
      "i_comments": 0,
      "i_likes": 0,
      "main_ds": "",
      "isubtype": 0,
      "subtype_desc": null,
      "created_epoch": 1674832244,
      "created_str": "20230127231044",
      "is_chosen": false,
      "priority": 0,
      "seq": 0,
      "generator": 1,
      "generator_model": null,
      "url_file_desc": "http://download.aboboo.com/media/courseware/picture/2023/01/27/186912.html",
      "url_file_cover": "",
      "privilege": 0,
      "search_tag": "",
      "version": 3,
      "picture": "/static/coursewares/default-thumbmail.gif",
      "thumbnail": "http://download.aboboo.com/media/courseware/thumbnail/2023/01/27/186912..png",
      "creator_name": "custerfun",
      "urls_file": [
        {
          "id": 187784,
          "url": "/api/0.1/coursewares/down/186912/",
          "name": "",
          "code": ""
        }
      ]
    
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci comment '项目表';

-- 课件句子
create table `dong`.groups_courseware_sentence (
    "text": "句子原文",
    "tran": "句子译文",
    "pron": "",
    "player1": "",
    "player2": "",
    "note": "",
    "num": 0,
    "bpos": 120,
    "epos": 3200,
    "checked": true,
    "html": "原文 html",
    "html_tr": "译文 html"
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci comment '项目表';

{
  "id": "English_MandarinS_613",
  "target_language": "English",
  "native_language": "MandarinS",
  "target_sentence": "We took a bus to the cinema and brought our own popcorn.",
  "native_sentence": "我们乘了公共汽车去电影院，还自带了爆米花。",
  "target_recording": "/audio/male/english/english_613.mp3",
  "native_recording": "/audio/male/mandarins/mandarins_613.mp3",
  "article": "",
  "pinyin": "",
  "furigana": "",
  "is_male": true,
  "difficulty": "Expert 2"
}

{"count":4012,"next":"https://www.aboboo.com/api/0.1/groups/search1/?format=json&kind=newest&max=10&page=2&page_size=50","previous":null,"results":[{"name":"点读精品课件","slogan":"专供会员使用","description":"点读精品课件小组。根据客户端的限制，只有vip会员才能下载使用点读课件。","img":"http://download.aboboo.com/media/groups/image/2023/03/14/zcq/resized/c1e1e2de_320.jpg","code":"1223167796","id":14221,"added_cw":"2023-03-22 18:24:58+0800","classifications":"100;101;103;104;105;106","rank":0,"cws":167,"mbs":19,"itype":1,"owner_name":"yong_lin"},{"name":"云忆精品课件组","slogan":"会员用精品","description":"收录云忆精品课件的小组。根据客户端的限制要求，非会员无法使用云忆课件。","img":"http://download.aboboo.com/media/groups/image/2023/03/14/zcq/resized/dcb204ba_320.png","code":"1016762164","id":14220,"added_cw":"2023-03-22 16:39:40+0800","classifications":"100;101;103;104;105;106","rank":0,"cws":137,"mbs":5,"itype":1,"owner_name":"yong_lin"},{"name":"沉浸晨读","slogan":"坚持日拱一卒","description":"以YLYK的晨读为主导资料，加入管理员的个人偏好的资料，共享学习","img":"http://download.aboboo.com/media/groups/image/2022/01/13/bonypony/resized/6dd33f7f_320.jpg","code":"441056840","id":12355,"added_cw":"2023-03-30 22:02:06+0800","classifications":"103;104;3;30;79;92","rank":0,"cws":272,"mbs":1773,"itype":0,"owner_name":"bonypony"},{"name":"yuklny","slogan":"加油!","description":"本课件是新概念英语，美音版本，本人基础也很差，希望大家能够一起学习，共同进步！","img":"http://download.aboboo.com/media/groups/image/2023/03/28/yuklny/resized/56a285c8_320.jpeg","code":"822382645","id":14285,"added_cw":"2023-03-30 21:10:22+0800","classifications":"101;103;105;3;62;91","rank":0,"cws":4,"mbs":0,"itype":0,"owner_name":"yuklny"},{"name":"广州版初中英语（沪教版）","slogan":"一起学习 共同进步","description":"便利学生学习英语 教材是广州用的沪教版的","img":"http://download.aboboo.com/media/groups/image/2021/09/06/dedely/resized/fc2f306d_320.png","code":"363097552","id":11768,"added_cw":"2023-03-30 20:36:46+0800","classifications":"103;104;105;21;3;91","rank":0,"cws":27,"mbs":30,"itype":0,"owner_name":"dedely"},{"name":"高考听力特训","slogan":"加油","description":"高中听力素材积累 包括历年高考真题模拟题","img":"http://download.aboboo.com/media/groups/image/2022/12/14/craigym/resized/746720df_320.jpg","code":"1092979390","id":13892,"added_cw":"2023-03-30 17:39:13+0800","classifications":"104;25;3;34;72;91","rank":0,"cws":18,"mbs":31,"itype":0,"owner_name":"craigym"},{"name":"精听。","slogan":"冲冲冲","description":"自己的文章随意添加。小文章精听，没有特别的主题","img":"http://download.aboboo.com/media/groups/image/2023/03/07/qqonce/resized/9f86a36a_320.png","code":"714510026","id":14191,"added_cw":"2023-03-30 16:20:10+0800","classifications":"103;104;3;30;72;92","rank":0,"cws":7,"mbs":25,"itype":0,"owner_name":"qqonce"},{"name":"零基础学英语","slogan":"每天学一点，水滴石穿。","description":"零基础，空闲时间较多。不需要提高多少，只是一个爱好。","img":"http://download.aboboo.com/media/groups/image/2023/01/02/armyking/resized/d6e3fd8f_320.jpg","code":"995887980","id":13951,"added_cw":"2023-03-30 14:18:43+0800","classifications":"101;103;105;3;62;88","rank":0,"cws":75,"mbs":480,"itype":0,"owner_name":"armyking"},{"name":"全国英语等级2 单词","slogan":"祝学渣变学霸","description":"啊啊啊，觉得有用的，给我点个抖音的关注吧，42349713 ，  谢谢","img":"http://download.aboboo.com/media/groups/image/2023/03/18/huangyz01/resized/acc20840_320.png","code":"850923394","id":14239,"added_cw":"2023-03-30 12:30:30+0800","classifications":"101;104;3;40;71;88","rank":0,"cws":7,"mbs":6,"itype":0,"owner_name":"huangyz01"},{"name":"基础模块2","slogan":"学习更好的完善自己","description":"有共同兴趣的老师完善课件，提供个性化的课件，让学生更好地提高自己","img":"http://download.aboboo.com/media/groups/image/2023/03/27/jinglingli_79/resized/677a1d8f_320.jpg","code":"702353433","id":14277,"added_cw":"2023-03-30 11:38:25+0800","classifications":"104;29;3;34;77;91","rank":0,"cws":6,"mbs":6,"itype":0,"owner_name":"jinglingli_79"},{"name":"Take Away Englsih","slogan":"一起学英语","description":"Take Away English ©British Broadcasting Corporation 2023\r\nbbclearningenglish.com/chinese","img":"http://download.aboboo.com/media/groups/image/2023/02/25/charlotte_cc/resized/0049e614_320.png","code":"894176792","id":14148,"added_cw":"2023-03-29 23:11:07+0800","classifications":"101;102;3;55;73;94","rank":0,"cws":2,"mbs":5,"itype":0,"owner_name":"charlotte_cc"},{"name":"7 Habits of Highly Effective People","slogan":"一起读书吧","description":"关于“高效能人士的七个习惯”，我们一起读书吧","img":"http://download.aboboo.com/media/groups/image/2022/05/05/acherking/resized/07d2bcd1_320.jpeg","code":"640319392","id":12932,"added_cw":"2023-03-29 23:08:12+0800","classifications":"101;102;104;3;82;93","rank":0,"cws":44,"mbs":176,"itype":0,"owner_name":"acherking"},{"name":"《西游记》英文版  Journey to the West","slogan":"lucky","description":"会说英语的齐天大圣！\r\n不同的配方，熟悉的味道，108集听力蜕变。","img":"http://download.aboboo.com/media/groups/image/2022/12/15/youxian99999/resized/e0f3d1d0_320.png","code":"798001868","id":13897,"added_cw":"2023-03-29 23:08:10+0800","classifications":"101;103;104;106;3;72","rank":0,"cws":76,"mbs":320,"itype":0,"owner_name":"youxian99999"},{"name":"日语五十音","slogan":"加油！","description":"日语五十音，只包括最基本的46个，资料来源于网络","img":"http://download.aboboo.com/media/groups/image/2023/03/28/guodongya/resized/b663ddf0_320.jpg","code":"1142165540","id":14279,"added_cw":"2023-03-29 23:07:55+0800","classifications":"101;103;104;5;71;92","rank":0,"cws":1,"mbs":0,"itype":0,"owner_name":"guodongya"},{"name":"Deutsch Nachrichtenleicht","slogan":"Good good study, day day up!","description":"Nachrichtenleicht Deutschstufe A2-B1","img":"http://download.aboboo.com/media/groups/image/2023/03/29/frankygl/resized/178b616f_320.png","code":"836341829","id":14286,"added_cw":"2023-03-29 19:39:13+0800","classifications":"101;103;104;105;7;93","rank":0,"cws":2,"mbs":0,"itype":0,"owner_name":"frankygl"},{"name":"新课标英语听力100篇","slogan":"适合我们中国应试的小学生提升听力水平","description":"非常适合我们中国应试的小学生提升听力水平。题型和内容都比较适合提升应试听力水平。有100篇听力，分为蜗牛篇，鹦鹉篇，百灵篇。含听力答案和原文，建议老师们在听力练习完让孩子们模仿听力原文的语音语调，注意连读，省音，声调，失去爆破等语音知识","img":"http://download.aboboo.com/media/groups/image/2023/03/27/simonlynch1/resized/0535bf33_320.png","code":"934711317","id":14273,"added_cw":"2023-03-29 16:43:38+0800","classifications":"103;104;14;3;72;90","rank":0,"cws":6,"mbs":1,"itype":0,"owner_name":"simonlynch1"},{"name":"葡萄柚的加拿大英语新闻听力练习","slogan":"学点是点，至少不会退步。","description":"来自加拿大的英语新闻。\n大概三四天更新一次。\n\n目前内容主要来自 CBC。\n以后会逐步加入加拿大其它媒体的新闻。\n\n对于新闻类的听力材料，我喜欢时长在一分钟左右的（最多不要超过两分钟）。现在提供的音频文件时长远超两分钟，但包括了数则新闻，你可以把每则新闻当作一个独立的单元来使用。\n\n音频文本通过音频转文字软件获得，可能会存在错误，大家使用时注意。\n\n现在 Aboboo 上美国英国的英语新闻听力材料极多，但加拿大的却极少。\n制作一些加拿大英语新闻听力材料，一方面锻炼听力，另一方面也能多了解一些加拿大的时事。\n希望能对在加拿大工作、学习的中国人提供些帮助。","img":"http://download.aboboo.com/media/groups/image/2023/02/20/grapefruit001/resized/78997072_320.png","code":"934677964","id":14123,"added_cw":"2023-03-29 00:45:19+0800","classifications":"101;103;104;3;72;93","rank":0,"cws":12,"mbs":25,"itype":0,"owner_name":"grapefruit001"},{"name":"测试小组","slogan":"测试小组加入无用","description":"测试小组加入无用测试小组加入无用测试小组加入无用","img":"http://download.aboboo.com/media/groups/image/2022/11/02/zcq/resized/e895a6ba_320.jpg","code":"752879554","id":13700,"added_cw":"2023-03-28 17:05:56+0800","classifications":"106;14;2;33;55;70","rank":0,"cws":2,"mbs":2,"itype":0,"owner_name":"zcq"},{"name":"哈尔的移动城堡","slogan":"頑張って","description":"第一次做课件，上传分享，请多多指教，希望大家也可以积极的创作，营造更好的日语学习环境！","img":"http://download.aboboo.com/media/groups/image/2023/03/10/huniiiiiilay/resized/e303610a_320.jpg","code":"846434049","id":14206,"added_cw":"2023-03-28 00:25:14+0800","classifications":"101;103;104;5;83;92","rank":0,"cws":9,"mbs":9,"itype":0,"owner_name":"huniiiiiilay"},{"name":"加州新版科学教材","slogan":"英语不是学科，是获取信息的工具","description":"加州新版科学教材Science A Closer Look(幼儿园、小学1-6年级教材)\nScience A Closer Look 是 McGraw-Hill 出版集团 出版的一套科学教材，文笔生动活泼，内容丰富多彩，图片精美绝伦，能激发小朋友对科学的浓厚兴趣。每个年级都配套有分级阅读资料 以G1为例：教材包含Life、Earth、Physical三大领域的内容","img":"http://download.aboboo.com/media/groups/image/2023/03/27/simonlynch1/resized/3e9fa3b9_320.png","code":"1075384336","id":14272,"added_cw":"2023-03-27 21:17:46+0800","classifications":"103;104;14;4;80;90","rank":0,"cws":36,"mbs":11,"itype":0,"owner_name":"simonlynch1"},{"name":"远旭英语1听力-5考试","slogan":"应试听力材料","description":"听力材料，KET、PET、雅思、托福、四六级、中高考等。","img":"http://download.aboboo.com/media/groups/image/2022/02/23/simonlynch1/resized/24b3f562_320.jpg","code":"596146282","id":12548,"added_cw":"2023-03-27 09:55:53+0800","classifications":"101;103;104;106;3;72","rank":0,"cws":27,"mbs":90,"itype":0,"owner_name":"simonlynch1"},{"name":"Golden Collection-英","slogan":"Growth in wisdom may be exactly measured by decrease in bitterness.","description":"Though much is taken, much abides; and though\nWe are not now that strength which in the old days\nMoved earth and heaven; that which we are, we are,\nOne equal-temper of heroic hearts,\nMade weak by time and fate, but strong in will\nTo strive, to seek, to find, and not to yield.","img":"http://download.aboboo.com/media/groups/image/2022/11/25/ice_md/resized/80f00d41_320.png","code":"893749618","id":13807,"added_cw":"2023-03-27 08:32:07+0800","classifications":"101;103;104;3;70;92","rank":0,"cws":13,"mbs":4,"itype":0,"owner_name":"ice_md"},{"name":"赖世雄美语   入门篇","slogan":"！","description":"本书课文内容涉及日常生活的各个方面，包括日常见面用语、电话用语、购物、旅游、求职以及社会性、历史性、哲理性文章等。","img":"http://download.aboboo.com/media/groups/image/2023/02/23/yymbydnsx/resized/1b659a43_320.jpg","code":"982355467","id":14139,"added_cw":"2023-03-26 23:50:28+0800","classifications":"101;103;3;58;73;88","rank":0,"cws":41,"mbs":34,"itype":0,"owner_name":"yymbydnsx"},{"name":"托福托福永远幸福","slogan":"Nothing Is Impossible","description":"托福真题 反复做 托福真题 反复做 托福真题 反复做","img":"http://download.aboboo.com/media/groups/image/2023/03/23/tingliwudidi/resized/5ae58698_320.gif","code":"793907137","id":14257,"added_cw":"2023-03-26 23:50:22+0800","classifications":"104;3;31;48;72;93","rank":0,"cws":2,"mbs":3,"itype":0,"owner_name":"tingliwudidi"},{"name":"学到宇宙爆炸","slogan":"学到宇宙爆炸","description":"自用课件，主要传一些博主Vlog，日常英语，主要提高口语表达，用机翻英文字幕，若有错误请大家指出来，感谢","img":"http://download.aboboo.com/media/groups/image/2023/02/18/laeli/resized/4cbf5ac6_320.jpg","code":"849546679","id":14116,"added_cw":"2023-03-26 23:49:31+0800","classifications":"101;103;104;3;76;92","rank":0,"cws":4,"mbs":18,"itype":0,"owner_name":"laeli"},{"name":"六级听力真题2016-2022年","slogan":"听力200+冲冲冲","description":"先更新2018-2022年的听力真题，后续更新较旧年份。\n听力部分题型：Conversation 长对话2个 + Passage 短文2个 + Recording 演讲3个","img":"http://download.aboboo.com/media/groups/image/2023/03/11/qianatang/resized/2e334739_320.jpg","code":"957746953","id":14209,"added_cw":"2023-03-26 10:49:38+0800","classifications":"104;3;30;37;72;92","rank":0,"cws":17,"mbs":165,"itype":0,"owner_name":"qianatang"},{"name":"NPR等英语新闻","slogan":"每天跟读10分钟","description":"英语（美语）新闻，了解美国社会新闻，每天英语跟读材料；主要摘录NPR新闻中的morning edition，一般为10分钟左右（周六周日无）。\n欢迎大家在对应的课件评论区留下自己的全文读的分数，让我们日拱一卒，不断进步。\n\n想要完成每日2#打卡任务的，不用多下载，一篇10分钟的材料就够了。\n\n跟读tip：\n如果觉得原文语速过快，可以先保证完整准确读完，之后慢慢提高语速。\n如果觉得句子太长，使劲跟一下，多读长句，之后再读其它短句，感觉so easy。\n部分有口音、有背景音乐的句子可以设置跳过。\n\n视时间允许，也会有其它的英语音视频材料，仅供英语学习。\n\n希望看到你们的跟读分数哦！大家互相鼓励，共同提高。\n如发现课件错误也请留言，谢谢！","img":"http://download.aboboo.com/media/groups/image/2023/02/09/znhskzj/resized/30abbfc2_320.png","code":"729615583","id":14052,"added_cw":"2023-03-25 22:45:55+0800","classifications":"101;103;104;106;3;79","rank":0,"cws":43,"mbs":168,"itype":0,"owner_name":"znhskzj"},{"name":"There Might Be Some Issues","slogan":"正视冲突，合作共赢","description":"忠诚合作，积极乐观，努力开拓、勇往直前。","img":"http://download.aboboo.com/media/groups/image/2022/03/29/nee67/resized/4fc0124c_320.jpg","code":"534805286","id":12748,"added_cw":"2023-03-25 21:16:03+0800","classifications":"104;29;3;34;73;91","rank":0,"cws":4,"mbs":1,"itype":0,"owner_name":"nee67"},{"name":"英语011","slogan":"英语","description":"自己用的初中英语学习资料自己用的初中英语学习资料","img":"http://download.aboboo.com/media/groups/image/2023/03/25/liu224/resized/fa036dac_320.jpg","code":"1126043631","id":14268,"added_cw":"2023-03-25 20:02:26+0800","classifications":"104;21;3;33;71;91","rank":0,"cws":1,"mbs":0,"itype":0,"owner_name":"liu224"},{"name":"剑雅真题听力","slogan":"8777","description":"主要是剑雅真题系列的听力原文，用于听写和跟读练习。","img":"http://download.aboboo.com/media/groups/image/2023/01/12/nathan_zhx/resized/4bb8c311_320.png","code":"660540386","id":13984,"added_cw":"2023-03-25 11:29:39+0800","classifications":"101;3;45;67;72;93","rank":0,"cws":14,"mbs":125,"itype":0,"owner_name":"nathan_zhx"},{"name":"历史上的今天（Today in history）","slogan":"听英语学历史","description":"Today in history （历史上的今天）是美联社打造的一档经典栏目，讲述历史上的今天发生的一些标志性、重大或影响深远的事情","img":"http://download.aboboo.com/media/groups/image/2023/01/14/simonlynch1/resized/c945dbda_320.png","code":"607913235","id":12546,"added_cw":"2023-03-25 01:24:16+0800","classifications":"101;103;104;106;3;80","rank":0,"cws":365,"mbs":521,"itype":0,"owner_name":"simonlynch1"},{"name":"BBC 6Minute English","slogan":"加油","description":"Learn and practise useful English language for everyday situations with the BBC. Your weekly instruction manual for saying or doing something in English is published every Thursday.","img":"http://download.aboboo.com/media/groups/image/2021/10/13/miiaen/resized/c9b56163_320.jpg","code":"589918244","id":11936,"added_cw":"2023-03-25 01:21:35+0800","classifications":"101;103;106;3;55;72","rank":0,"cws":83,"mbs":1333,"itype":0,"owner_name":"miiaen"},{"name":"Various English Materials","slogan":"Day Day Up","description":"Good Good Study, Day Day Up.\n材料主要来源于目前学习的网站 British Council、ESL、Economists。\n译文多为机翻，偶尔会修改。","img":"http://download.aboboo.com/media/groups/image/2022/08/15/nabiyoo/resized/4f753ca7_320.jpg","code":"1103299437","id":13368,"added_cw":"2023-03-25 01:20:20+0800","classifications":"103;104;3;30;72;92","rank":0,"cws":62,"mbs":88,"itype":0,"owner_name":"nabiyoo"},{"name":"oErica学习日记","slogan":"冲鸭~","description":"自律的顶端就是享受孤独，看似不起波澜的日复一日，会突然在某天，让人看到坚持的意义。","img":"http://download.aboboo.com/media/groups/image/2022/05/17/oerica/resized/202dff31_320.jpg","code":"353992364","id":13007,"added_cw":"2023-03-24 11:15:27+0800","classifications":"101;103;104;3;72;92","rank":0,"cws":47,"mbs":27,"itype":0,"owner_name":"oerica"},{"name":"新概念加油","slogan":"加油","description":"新概念背诵加油, 分段背诵新概念, 句句啃下新概念.","img":"http://download.aboboo.com/media/groups/image/2022/11/22/wsttfutures/resized/41f74c99_320.jpg","code":"729155892","id":13790,"added_cw":"2023-03-23 10:15:26+0800","classifications":"101;103;105;3;62;94","rank":0,"cws":7,"mbs":62,"itype":0,"owner_name":"wsttfutures"},{"name":"Vic oring","slogan":"O-ring 密封圈","description":"首先是用于自学的，不喜勿扰。编辑课件还在学习中，还需要打磨。\n***以最严格的要求从最基础的地方做起\n    最聪明的人一定要下最笨的功夫\n    前面就是地狱\n    在路口处要根绝一切犹豫\n    出来就是炼狱***","img":"http://download.aboboo.com/media/groups/image/2023/03/17/vicoring/resized/f0d13242_320.jpg","code":"993857396","id":14236,"added_cw":"2023-03-23 00:12:59+0800","classifications":"104;3;30;36;72;92","rank":0,"cws":4,"mbs":3,"itype":0,"owner_name":"vicoring"},{"name":"看图学英语-无图版","slogan":"锻炼英语思维","description":"该课件论坛里已经有了女声版的，本次发该课件，只是为了熟悉原来的语句。\n男声版目的只是为了可以听到多种不同的语音\n对已经有的女声版里的错误进行了修正，断句个人觉得不太合理的重新做了断句。\n更新速度就是本人学习进度，文字为听写录入。","img":"http://download.aboboo.com/media/groups/image/2021/12/19/yiwu_wang/resized/c7901ed7_320.jpg","code":"360313977","id":12269,"added_cw":"2023-03-23 00:12:52+0800","classifications":"101;103;104;3;73;88","rank":0,"cws":59,"mbs":86,"itype":0,"owner_name":"yiwu_wang"},{"name":"1000 Basic English Words","slogan":"1000 Basic English Words，需要的书友尽情下载吧","description":"1000 Basic English Words是是一本四本书组成的系列丛书，旨在提高初学者的词汇量。他提供了1000多个实用的高频英语单词，这些单词是在对英语学习者的课本和材料中最常见的单词进行分析后挑选出来的。","img":"http://download.aboboo.com/media/groups/image/2023/03/21/shmilyxy1983/resized/de9066a2_320.jpg","code":"1093504947","id":14251,"added_cw":"2023-03-22 10:04:41+0800","classifications":"103;104;14;3;71;90","rank":0,"cws":14,"mbs":17,"itype":0,"owner_name":"shmilyxy1983"},{"name":"英语时文--阅读理解【中考】","slogan":"与”时“俱进，与”文“偕行！","description":"聚焦当前国内外的热点话题和事件，选材大部分源于外刊、外网，文章新颖有趣，训练学生英语阅读能力的同时，还能帮助学生拓宽视野、增长知识。","img":"http://download.aboboo.com/media/groups/image/2022/03/26/lgw2020/resized/427effdc_320.jpg","code":"591624939","id":12726,"added_cw":"2023-03-21 22:20:48+0800","classifications":"104;106;21;3;33;70","rank":0,"cws":2,"mbs":14,"itype":0,"owner_name":"lgw2020"},{"name":"One story a day for beginners","slogan":"One story a day for beginners，需要的学友们尽情下载！！","description":"One story a day for beginners，需要的学友们尽情下载！！","img":"http://download.aboboo.com/media/groups/image/2023/03/14/shmilyxy1983/resized/05f0a2f5_320.jpg","code":"835948339","id":14219,"added_cw":"2023-03-21 12:40:50+0800","classifications":"103;104;105;14;3;89","rank":0,"cws":31,"mbs":38,"itype":0,"owner_name":"shmilyxy1983"},{"name":"新闻听写","slogan":"The more I got, the more amazed I was by the river.","description":"英语新闻为主 VOA CNN NPR TED...","img":"http://download.aboboo.com/media/groups/image/2022/12/27/estelleliuyi/resized/783a2477_320.jpeg","code":"949881655","id":13928,"added_cw":"2023-03-20 17:27:18+0800","classifications":"101;103;104;3;72;92","rank":0,"cws":5,"mbs":82,"itype":0,"owner_name":"estelleliuyi"},{"name":"新概念英语第三册美音版精校版","slogan":"教材不能有错，有错必须改正","description":"我只想要一个美音版的、没有错误的新概念英语三课件组。目前小组里面的要么是英音版，要么是美音版但是几乎每个课件都有错误的。\n于是不得已，只能自己做一个。也没有什么崇高的想法，就想着自己后面还要反复听，至少要方便自己吧。\n朋友们如果发现有错误的，及时提醒我，我改完重新上传更新。\n本组课件基本上基于 sgxxm 的《新概念美音版-第三册》","img":"http://download.aboboo.com/media/groups/image/2022/06/08/xjr7670/resized/a3bcde9d_320.jpg","code":"396285265","id":13106,"added_cw":"2023-03-20 12:00:17+0800","classifications":"28;3;36;62;70;91","rank":0,"cws":60,"mbs":1132,"itype":0,"owner_name":"xjr7670"},{"name":"航空二年级1班","slogan":"云端之上 蓝色梦想","description":"空乘口语 机上服务 航空英语 从起飞到落地","img":"http://download.aboboo.com/media/groups/image/2023/03/19/xinyinlili/resized/877ebf78_320.jpg","code":"1067716493","id":14243,"added_cw":"2023-03-20 11:31:27+0800","classifications":"101;103;104;3;73;88","rank":0,"cws":1,"mbs":3,"itype":0,"owner_name":"xinyinlili"},{"name":"黑猫英语分级读物 --【中学A级】","slogan":"原典诵讲；非同凡响。","description":"黑猫英语分级读物【中学段】分为【A-F】6个级别，共有66本；以经典名著改编本为主，亦含少量当代原创作品。","img":"http://download.aboboo.com/media/groups/image/2021/11/26/lgw2020/resized/302b2876_320.jpg","code":"471462673","id":12165,"added_cw":"2023-03-19 21:07:12+0800","classifications":"104;21;3;33;70;91","rank":0,"cws":2,"mbs":27,"itype":0,"owner_name":"lgw2020"},{"name":"新概念英语第四册美音版精校版","slogan":"教材不能有错，有错必须改正","description":"我只想要一个美音版的、没有错误的新概念英语四课件组。目前小组里面的要么是英音版，要么是美音版但是几乎每个课件都有错误的。\n于是不得已，只能自己做一个。也没有什么崇高的想法，就想着自己后面还要反复听，至少要方便自己吧。\n朋友们如果发现有错误的，及时提醒我，我改完重新上传更新。\n本组课件基本上基于 sgxxm 的《新概念美音版-第四册》","img":"http://download.aboboo.com/media/groups/image/2022/07/26/xjr7670/resized/7bc78984_320.png","code":"1095860951","id":13315,"added_cw":"2023-03-19 17:56:14+0800","classifications":"3;30;37;62;72;92","rank":0,"cws":48,"mbs":359,"itype":0,"owner_name":"xjr7670"},{"name":"英语口语初级","slogan":"英语口语初级","description":"外研社英语口语权威教程-初级口语......","img":"http://download.aboboo.com/media/groups/image/2023/03/13/cx_0911/resized/e0e58f5f_320.jpg","code":"992710436","id":14217,"added_cw":"2023-03-18 23:42:11+0800","classifications":"101;103;104;3;73;89","rank":0,"cws":5,"mbs":20,"itype":0,"owner_name":"cx_0911"},{"name":"阅读和欣赏","slogan":"阅读和欣赏","description":"阅读和欣赏\n古典文学爱好者，愿意阅读和欣赏一辈子。","img":"http://download.aboboo.com/media/groups/image/2022/01/03/kkqqv/resized/580bdec4_320.jpg","code":"639792428","id":12323,"added_cw":"2023-03-18 01:01:03+0800","classifications":"101;103;104;105;106;2","rank":0,"cws":18,"mbs":44,"itype":0,"owner_name":"kkqqv"},{"name":"PTE RA 跟读-高分示范","slogan":"let's shadowing together!!!","description":"PTE RA影子跟读，口语迅速提升，大家一起努力拿到口语90分！！！","img":"http://download.aboboo.com/media/groups/image/2023/03/05/jannerf/resized/567e433b_320.jpg","code":"652971708","id":14185,"added_cw":"2023-03-17 01:00:15+0800","classifications":"101;102;104;3;73;93","rank":0,"cws":22,"mbs":74,"itype":0,"owner_name":"jannerf"},{"name":"PTE RA 跟读-高分示范慢速版","slogan":"let's shadowing together","description":"PTE RA 高分示范慢速版，影子跟读入门首选","img":"http://download.aboboo.com/media/groups/image/2023/03/05/jannerf/resized/567e433b_NKGOlf8_320.jpg","code":"974032573","id":14184,"added_cw":"2023-03-17 01:00:13+0800","classifications":"101;102;104;105;3;93","rank":0,"cws":22,"mbs":53,"itype":0,"owner_name":"jannerf"},{"name":"远旭英语1口语-3表达","slogan":"不同表达练习","description":"日常表达的常用词组、句子的专项练习记忆。","img":"http://download.aboboo.com/media/groups/image/2022/02/23/simonlynch1/resized/1f877d0d_320.jpg","code":"563869801","id":12547,"added_cw":"2023-03-16 08:38:26+0800","classifications":"101;103;104;3;73;93","rank":0,"cws":88,"mbs":73,"itype":0,"owner_name":"simonlynch1"}],"page_size":50}