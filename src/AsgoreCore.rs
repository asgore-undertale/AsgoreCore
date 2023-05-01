// use std::time::Instant;
use std::{fs, cmp};
use std::io::{Write, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use const_format::concatcp;
use clipboard::{ClipboardContext, ClipboardProvider};

// ======================= Global Consts

const A_SEPARATOR: &str = "█";
const AFF_FILLER: &str = "•";
const AFF_MIN_SEPARATOR: &str = "/";
const CSV_DELIMITER: char = ',';
// const CSV_QUOTECHAR: char = '\"';
// const ZTA_RANGE: &str = "{:}"; // zta : zip two arrays  // zts : zip two strings

const ACT_HEADER: &str = concatcp!("الحرف", A_SEPARATOR, "منفصل", A_SEPARATOR, "متصل بما قبله", A_SEPARATOR, "متصل بما بعده", A_SEPARATOR, "متصل من الجهتين");
const AFF_HEADER: &str = concatcp!("Char", A_SEPARATOR, "X", A_SEPARATOR, "Y", A_SEPARATOR, "Width", A_SEPARATOR, "Height", A_SEPARATOR, "Xoffset", A_SEPARATOR, "Yoffset", A_SEPARATOR, "Xadvance");
const TRANSLATION_SHEET_HEADER: &str = concatcp!("الإزاحة بالعشري", CSV_DELIMITER, "النص الأصلي", CSV_DELIMITER, "الترجمة");

const ARABIC_CHARS: &str = "ٱٻپڀٺٿٹڤڦڄڃچڇڍڌڎڈژڑکگڳڱںڻۀہھےۓڭۇۆۈۋۅۉېیءآأؤإئابةتثجحخدذرزسشصضطظعغفقكلمنهوىيـ"; // and Persian
const FREEZED_ARABIC_CHARS: &str = "ﺀﺁﺂﺃﺄﺅﺆﺇﺈﺉﺊﺋﺌﺍﺎﺏﺐﺑﺒﺓﺔﺕﺖﺗﺘﺙﺚﺛﺜﺝﺞﺟﺠﺡﺢﺣﺤﺥﺦﺧﺨﺩﺪﺫﺬﺭﺮﺯﺰﺱﺲﺳﺴﺵﺶﺷﺸﺹﺺﺻﺼﺽﺾﺿﻀﻁﻂﻃﻄﻅﻆﻇﻈﻉﻊﻋﻌﻍﻎﻏﻐﻑﻒﻓﻔﻕﻖﻗﻘﻙﻚﻛﻜﻝﻞﻟﻠﻡﻢﻣﻤﻥﻦﻧﻨﻩﻪﻫﻬﻭﻮﻯﻰﻱﻲﻳﻴﻵﻶﻷﻸﻹﻺﻻﻼﭐﭑﭑﭐﭔﭕﭓﭒﭘﭙﭗﭖﭜﭝﭛﭚﭠﭡﭟﭞﭤﭥﭣﭢﭨﭩﭧﭦﭬﭭﭫﭪﭰﭱﭯﭮﭴﭵﭳﭲﭸﭹﭷﭶﭼﭽﭻﭺﮀﮁﭿﭾﮂﮃﮃﮂﮄﮅﮅﮄﮆﮇﮇﮆﮈﮉﮉﮈﮊﮋﮋﮊﮌﮍﮍﮌﮐﮑﮏﮎﮔﮕﮓﮒﮘﮙﮗﮖﮜﮝﮛﮚﯨﯩﮟﮞﮢﮣﮡﮠﮤﮥﮥﮤﮨﮩﮧﮦﮬﮭﮫﮪﮮﮯﮯﮮﮰﮱﮱﮰﯕﯖﯔﯓﯗﯘﯘﯗﯙﯚﯚﯙﯛﯜﯜﯛﯞﯟﯟﯞﯠﯡﯡﯠﯢﯣﯣﯢﯦﯧﯥﯤﯾﯿﯽﯼ";
const CHARS_CONNECT_BEFORE: &str = "آأؤإاةدذرزوىٱڍڌڎڈژڑۀےۓۇۆۈۋۅۉ";
const CHARS_CONNECT_BOTH: &str = "ئبتثجحخسشصضطظعغفقكلمنهيـٻپڀٺٿٹڤڦڄڃچڇکگڳڱںڻہھڭېی";
const HARAKAT: &str = "ًٌٍَُِّْ";
const FREEZED_HARAKAT: &str = "ﱞﱟﱠﱡﱢﱣﳲﳳﳴﴼﴽﹰﹱﹲﹴﹶﹷﹸﹹﹺﹻﹼﹽﹾﹿ";
const ASCII: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const EXTENDED_ASCII: &str = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
const SYMBOLS: &str = " !*+-.:=|~\"'#$%&";
const ARABIC_SYMBOLS: &str = "؟،؛ـ";
const RETURNS: &str = "\r\n";

const BRACKETS_LIST: [&str; 33] = [
	"()", "<>", "[]", "{}", "❨❩", "❪❫", "❬❭", "❮❯", "❰❱", "❲❳", "❴❵", "⟅⟆", "⟦⟧", "⟨⟩", "⟪⟫", "⟬⟭", "⟮⟯", "⦅⦆", "⦋⦌", "⦍⦎", "⦑⦒", "⦗⦘", "⧼⧽", "〈〉", "《》", "「」", "『』", "【】", "〔〕", "〖〗", "〘〙", "〚〛", "＜＞"
];

const NOT_LAST_IN_LINE: [&str; 62] = [
    "من", "إلى", "على", "ثم", "حتى", "بدون", "دون", "لن", "إذن", "لما", "كأن", "لكن", "ليت", "لعل", "ما", "لو", "إذا", "يا", "أيا", "هيا",
    "هل", "مع", "الذي", "التي", "الذان", "اللتان", "الذين", "اللواتي", "اللاتي", "اللائي", "و", "أم", "أن", "إن", "أو", "أي", "إي", "بل",
    "عن", "في", "قد", "لقد", "كي", "لا", "لم", "مذ", "منذ", "ها", "وا", "وي", "ألا", "جير", "خلا", "رب", "سوف", "عدا", "إلا", "إما", "أما",
    "هلا", "لولا", "لوما"
];

lazy_static! {
	static ref MERGED_HARAKAT_MAP: HashMap<&'static str, Vec<String>> = [
		("ﱠ" , vec![String::from("ّﹼ"), String::from("َﹶ")]), ("ﳲ", vec![String::from("ﹽ"), String::from("َﹶﹷ"), String::from("ّﹼﹽ"), String::from("ﹷ")]), ("ﱡ", vec![String::from("ّﹼ"), String::from("ُﹸ")]), 
		("ﳳ", vec![String::from("ﹽ"), String::from("ُﹸﹹ"), String::from("ّﹼﹽ"), String::from("ﹹ")]), ("ﱢ", vec![String::from("ّﹼ"), String::from("ِﹺ")]), ("ﳴ", vec![String::from("ﹽ"), String::from("ِﹺﹻ"), String::from("ّﹼﹽ"), String::from("ﹻ")]),
		("ﱞ" , vec![String::from("ّﹼ"), String::from("ٌﹲ")]), ("ﱟ", vec![String::from("ّﹼ"), String::from("ٍﹴ")])
	].iter().cloned().collect();
	
    static ref CONNECTED_HARAKAT_MAP: HashMap<char, char> = [
		('ﹶ', 'ﹷ'), ('َ', 'ﹷ'), ('ﹸ', 'ﹹ'), ('ُ', 'ﹹ'), ('ﹺ', 'ﹻ'),
		('ِ', 'ﹻ'), ('ﹼ', 'ﹽ'), ('ّ', 'ﹽ'), ('ﹾ', 'ﹿ'), ('ْ', 'ﹿ'),
		('ﱟ', 'ﹱ'), ('ﹰ', 'ﹱ'), ('ً', 'ﹱ'), ('ﱠ', 'ﳲ'), ('ﱡ', 'ﳳ'),
		('ﱢ', 'ﳴ')
	].iter().cloned().collect();
	
	static ref FREEZED_ARABIC_MAP: HashMap<char, [char; 4]> = [
		// <منفصل><متصل بما قبله><متصل بما بعده><متصل من الجهتين>
		('ء', ['ﺀ', 'ﺀ', 'ﺀ', 'ﺀ']), ('آ', ['ﺁ', 'ﺂ', 'ﺁ', 'ﺂ']), ('أ', ['ﺃ', 'ﺄ', 'ﺃ', 'ﺄ']), ('ؤ', ['ﺅ', 'ﺆ', 'ﺅ', 'ﺆ']), ('إ', ['ﺇ', 'ﺈ', 'ﺇ', 'ﺈ']),
		('ئ', ['ﺉ', 'ﺊ', 'ﺋ', 'ﺌ']), ('ا', ['ﺍ', 'ﺎ', 'ﺍ', 'ﺎ']), ('ب', ['ﺏ', 'ﺐ', 'ﺑ', 'ﺒ']), ('ة', ['ﺓ', 'ﺔ', 'ﺓ', 'ﺔ']), ('ت', ['ﺕ', 'ﺖ', 'ﺗ', 'ﺘ']),
		('ث', ['ﺙ', 'ﺚ', 'ﺛ', 'ﺜ']), ('ج', ['ﺝ', 'ﺞ', 'ﺟ', 'ﺠ']), ('ح', ['ﺡ', 'ﺢ', 'ﺣ', 'ﺤ']), ('خ', ['ﺥ', 'ﺦ', 'ﺧ', 'ﺨ']), ('د', ['ﺩ', 'ﺪ', 'ﺩ', 'ﺪ']),
		('ذ', ['ﺫ', 'ﺬ', 'ﺫ', 'ﺬ']), ('ر', ['ﺭ', 'ﺮ', 'ﺭ', 'ﺮ']), ('ز', ['ﺯ', 'ﺰ', 'ﺯ', 'ﺰ']), ('س', ['ﺱ', 'ﺲ', 'ﺳ', 'ﺴ']), ('ش', ['ﺵ', 'ﺶ', 'ﺷ', 'ﺸ']),
		('ص', ['ﺹ', 'ﺺ', 'ﺻ', 'ﺼ']), ('ض', ['ﺽ', 'ﺾ', 'ﺿ', 'ﻀ']), ('ط', ['ﻁ', 'ﻂ', 'ﻃ', 'ﻄ']), ('ظ', ['ﻅ', 'ﻆ', 'ﻇ', 'ﻈ']), ('ع', ['ﻉ', 'ﻊ', 'ﻋ', 'ﻌ']),
		('غ', ['ﻍ', 'ﻎ', 'ﻏ', 'ﻐ']), ('ف', ['ﻑ', 'ﻒ', 'ﻓ', 'ﻔ']), ('ق', ['ﻕ', 'ﻖ', 'ﻗ', 'ﻘ']), ('ك', ['ﻙ', 'ﻚ', 'ﻛ', 'ﻜ']), ('ل', ['ﻝ', 'ﻞ', 'ﻟ', 'ﻠ']),
		('م', ['ﻡ', 'ﻢ', 'ﻣ', 'ﻤ']), ('ن', ['ﻥ', 'ﻦ', 'ﻧ', 'ﻨ']), ('ه', ['ﻩ', 'ﻪ', 'ﻫ', 'ﻬ']), ('و', ['ﻭ', 'ﻮ', 'ﻭ', 'ﻮ']), ('ى', ['ﻯ', 'ﻰ', 'ﻯ', 'ﻰ']),
		('ي', ['ﻱ', 'ﻲ', 'ﻳ', 'ﻴ']), ('ً' , ['ﹰ', 'ﹰ', 'ﹰ', 'ﹰ']), ('ٌ' , ['ﹲ', 'ﹲ', 'ﹲ', 'ﹲ']), ('ٍ' , ['ﹴ', 'ﹴ', 'ﹴ', 'ﹴ']), ('َ' , ['ﹶ', 'ﹶ', 'ﹶ', 'ﹶ']),
		('ُ' , ['ﹸ', 'ﹸ', 'ﹸ', 'ﹸ']), ('ِ' , ['ﹺ', 'ﹺ', 'ﹺ', 'ﹺ']), ('ّ' , ['ﹼ', 'ﹼ', 'ﹼ', 'ﹼ']), ('ْ' , ['ﹾ', 'ﹾ', 'ﹾ', 'ﹾ']), ('ٱ', ['ﭐ', 'ﭑ', 'ﭐ', 'ﭑ']),
		('ٹ', ['ﭦ', 'ﭧ', 'ﭨ', 'ﭩ']), ('ٺ', ['ﭞ', 'ﭟ', 'ﭠ', 'ﭡ']), ('ٻ', ['ﭒ', 'ﭓ', 'ﭔ', 'ﭕ']), ('پ', ['ﭖ', 'ﭗ', 'ﭘ', 'ﭙ']), ('پ', ['ﭖ', 'ﭗ', 'ﭘ', 'ﭙ']),
		('ٿ', ['ﭢ', 'ﭣ', 'ﭤ', 'ﭥ']), ('ڀ', ['ﭚ', 'ﭛ', 'ﭜ', 'ﭝ']), ('ڃ', ['ﭶ', 'ﭷ', 'ﭸ', 'ﭹ']), ('ڄ', ['ﭲ', 'ﭳ', 'ﭴ', 'ﭵ']), ('چ', ['ﭺ', 'ﭻ', 'ﭼ', 'ﭽ']),
		('چ', ['ﭺ', 'ﭻ', 'ﭼ', 'ﭽ']), ('ڇ', ['ﭾ', 'ﭿ', 'ﮀ', 'ﮁ']), ('ڈ', ['ﮈ', 'ﮉ', 'ﮈ', 'ﮉ']), ('ڌ', ['ﮄ', 'ﮅ', 'ﮄ', 'ﮅ']), ('ڍ', ['ﮂ', 'ﮃ', 'ﮂ', 'ﮃ']),
		('ڎ', ['ﮆ', 'ﮇ', 'ﮆ', 'ﮇ']), ('ڑ', ['ﮌ', 'ﮍ', 'ﮌ', 'ﮍ']), ('ژ', ['ﮊ', 'ﮋ', 'ﮊ', 'ﮋ']), ('ڤ', ['ﭪ', 'ﭫ', 'ﭬ', 'ﭭ']), ('ڤ', ['ﭪ', 'ﭫ', 'ﭬ', 'ﭭ']),
		('ڦ', ['ﭮ', 'ﭯ', 'ﭰ', 'ﭱ']), ('ک', ['ﮎ', 'ﮏ', 'ﮐ', 'ﮑ']), ('ڭ', ['ﯓ', 'ﯔ', 'ﯕ', 'ﯖ']), ('گ', ['ﮒ', 'ﮓ', 'ﮔ', 'ﮕ']), ('ڱ', ['ﮚ', 'ﮛ', 'ﮜ', 'ﮝ']),
		('ڳ', ['ﮖ', 'ﮗ', 'ﮘ', 'ﮙ']), ('ں', ['ﮞ', 'ﮟ', 'ﯨ', 'ﯩ']), ('ڻ', ['ﮠ', 'ﮡ', 'ﮢ', 'ﮣ']), ('ھ', ['ﮪ', 'ﮫ', 'ﮬ', 'ﮭ']), ('ۀ', ['ﮤ', 'ﮥ', 'ﮤ', 'ﮥ']),
		('ہ', ['ﮦ', 'ﮧ', 'ﮨ', 'ﮩ']), ('ۅ', ['ﯠ', 'ﯡ', 'ﯠ', 'ﯡ']), ('ۆ', ['ﯙ', 'ﯚ', 'ﯙ', 'ﯚ']), ('ۇ', ['ﯗ', 'ﯘ', 'ﯗ', 'ﯘ']), ('ۈ', ['ﯛ', 'ﯜ', 'ﯛ', 'ﯜ']),
		('ۉ', ['ﯢ', 'ﯣ', 'ﯢ', 'ﯣ']), ('ۋ', ['ﯞ', 'ﯟ', 'ﯞ', 'ﯟ']), ('ی', ['ﯼ', 'ﯽ', 'ﯾ', 'ﯿ']), ('ې', ['ﯤ', 'ﯥ', 'ﯦ', 'ﯧ']), ('ے', ['ﮮ', 'ﮯ', 'ﮮ', 'ﮯ']),
		('ۓ', ['ﮰ', 'ﮱ', 'ﮰ', 'ﮱ'])
	].iter().cloned().collect();
	
	pub static ref MERGED_ARABIC_MAP: HashMap<char, &'static str> = [
		('ﻵ', "ﻟﺂ"), ('ﻶ', "ﻠﺂ"), ('ﻷ', "ﻟﺄ"), ('ﻸ', "ﻠﺄ"), ('ﻹ', "ﻟﺈ"), ('ﻺ', "ﻠﺈ"), ('ﻻ', "ﻟﺎ"), ('ﻼ', "ﻠﺎ"),
		('ﯪ', "ﺋﺎ"), ('ﯫ', "ﺌﺎ"), ('ﯬ', "ﺋﻪ"), ('ﯭ', "ﺌﻪ"), ('ﯮ', "ﺋﻮ"), ('ﯯ', "ﺌﻮ"), ('ﰄ', "ﺋﻲ"), ('ﱩ', "ﺌﻲ"), ('ﯸ', "ﺋﻴ"), ('ﰃ', "ﺋﻰ"),
		('ﱨ', "ﺌﻰ"), ('ﰀ', "ﺋﺞ"), ('ﰁ', "ﺋﺢ"), ('ﰂ', "ﺋﻢ"), ('ﰅ', "ﺑﺞ"), ('ﰆ', "ﺑﺢ"), ('ﰇ', "ﺑﺦ"), ('ﰈ', "ﺑﻢ"), ('ﰉ', "ﺑﻰ"), ('ﰊ', "ﺑﻲ"),
		('ﰋ', "ﺗﺞ"), ('ﰌ', "ﺗﺢ"), ('ﰍ', "ﺗﺦ"), ('ﰎ', "ﺗﻢ"), ('ﰏ', "ﺗﻰ"), ('ﰐ', "ﺗﻲ"), ('ﰑ', "ﺛﺞ"), ('ﰒ', "ﺛﻢ"), ('ﰓ', "ﺛﻰ"), ('ﰔ', "ﺛﻲ"),
		('ﰕ', "ﺟﺢ"), ('ﰖ', "ﺟﻢ"), ('ﰗ', "ﺣﺞ"), ('ﰘ', "ﺣﻢ"), ('ﰙ', "ﺧﺞ"), ('ﰚ', "ﺧﺢ"), ('ﰛ', "ﺧﻢ"), ('ﰜ', "ﺳﺞ"), ('ﰝ', "ﺳﺢ"), ('ﰞ', "ﺳﺦ"),
		('ﰟ', "ﺳﻢ"), ('ﰠ', "ﺻﺢ"), ('ﰡ', "ﺻﻢ"), ('ﰢ', "ﺿﺞ"), ('ﰣ', "ﺿﺢ"), ('ﰤ', "ﺿﺦ"), ('ﰥ', "ﺿﻢ"), ('ﰦ', "ﻃﺢ"), ('ﰧ', "ﻃﻢ"), ('ﰨ', "ﻇﻢ"),
		('ﰩ', "ﻋﺞ"), ('ﰪ', "ﻋﻢ"), ('ﰫ', "ﻏﺞ"), ('ﰬ', "ﻏﻢ"), ('ﰭ', "ﻓﺞ"), ('ﰮ', "ﻓﺢ"), ('ﰯ', "ﻓﺦ"), ('ﰰ', "ﻓﻢ"), ('ﰱ', "ﻓﻰ"), ('ﰲ', "ﻓﻲ"),
		('ﰳ', "ﻗﺢ"), ('ﰴ', "ﻗﻢ"), ('ﰵ', "ﻗﻰ"), ('ﰶ', "ﻗﻲ"), ('ﰷ', "ﻛﺎ"), ('ﰸ', "ﻛﺞ"), ('ﰹ', "ﻛﺢ"), ('ﰺ', "ﻛﺦ"), ('ﰻ', "ﻛﻞ"), ('ﰼ', "ﻛﻢ"),
		('ﰽ', "ﻛﻰ"), ('ﰾ', "ﻛﻲ"), ('ﰿ', "ﻟﺞ"), ('ﱀ', "ﻟﺢ"), ('ﱁ', "ﻟﺦ"), ('ﱂ', "ﻟﻢ"), ('ﱃ', "ﻟﻰ"), ('ﱄ', "ﻟﻲ"), ('ﱅ', "ﻣﺞ"), ('ﱆ', "ﻣﺢ"),
		('ﱇ', "ﻣﺦ"), ('ﱈ', "ﻣﻢ"), ('ﱉ', "ﻣﻰ"), ('ﱊ', "ﻣﻲ"), ('ﱋ', "ﻧﺞ"), ('ﱌ', "ﻧﺢ"), ('ﱍ', "ﻧﺦ"), ('ﱎ', "ﻧﻢ"), ('ﱏ', "ﻧﻰ"), ('ﱐ', "ﻧﻲ"),
		('ﱑ', "ﻫﺞ"), ('ﱒ', "ﻫﻢ"), ('ﱓ', "ﻫﻰ"), ('ﱔ', "ﻫﻲ"), ('ﱕ', "ﻳﺞ"), ('ﱖ', "ﻳﺢ"), ('ﱗ', "ﻳﺦ"), ('ﱘ', "ﻳﻢ"), ('ﱙ', "ﻳﻰ"), ('ﱚ', "ﻳﻲ"),
		('ﱞ', "ﹼﹲ"), ('ﱟ', "ﹼﹴ"), ('ﱠ', "ﹼﹶ"), ('ﱡ', "ﹼﹸ"), ('ﱢ', "ﹼﹺ"), ('ﱤ', "ﺌﺮ"), ('ﱥ', "ﺌﺰ"), ('ﱦ', "ﺌﻢ"), ('ﱧ', "ﺌﻦ"), ('ﱪ', "ﺒﺮ"),
		('ﱫ', "ﺒﺰ"), ('ﱬ', "ﺒﻢ"), ('ﱭ', "ﺒﻦ"), ('ﱮ', "ﺒﻰ"), ('ﱯ', "ﺒﻲ"), ('ﱰ', "ﺘﺮ"), ('ﱱ', "ﺘﺰ"), ('ﱲ', "ﺘﻢ"), ('ﱳ', "ﺘﻦ"), ('ﱴ', "ﺘﻰ"),
		('ﱵ', "ﺘﻲ"), ('ﱶ', "ﺜﺮ"), ('ﱷ', "ﺜﺰ"), ('ﱸ', "ﺜﻢ"), ('ﱹ', "ﺜﻦ"), ('ﱺ', "ﺜﻰ"), ('ﱻ', "ﺜﻲ"), ('ﱼ', "ﻔﻰ"), ('ﱽ', "ﻔﻲ"), ('ﱾ', "ﻘﻰ"),
		('ﱿ', "ﻘﻲ"), ('ﲀ', "ﻜﺎ"), ('ﲁ', "ﻜﻞ"), ('ﲂ', "ﻜﻢ"), ('ﲃ', "ﻜﻰ"), ('ﲄ', "ﻜﻲ"), ('ﲅ', "ﻠﻢ"), ('ﲆ', "ﻠﻰ"), ('ﲇ', "ﻠﻲ"), ('ﲈ', "ﻤﺎ"),
		('ﲉ', "ﻤﻢ"), ('ﲊ', "ﻨﺮ"), ('ﲋ', "ﻨﺰ"), ('ﲌ', "ﻨﻢ"), ('ﲍ', "ﻨﻦ"), ('ﲎ', "ﻨﻰ"), ('ﲏ', "ﻨﻲ"), ('ﲑ', "ﻴﺮ"), ('ﲒ', "ﻴﺰ"), ('ﲓ', "ﻴﻢ"),
		('ﲔ', "ﻴﻦ"), ('ﲕ', "ﻴﻰ"), ('ﲖ', "ﻴﻲ"), ('ﲗ', "ﺋﺠ"), ('ﲘ', "ﺋﺤ"), ('ﲙ', "ﺋﺨ"), ('ﲚ', "ﺋﻤ"), ('ﲛ', "ﺋﻬ"), ('ﲜ', "ﺑﺠ"), ('ﲝ', "ﺑﺤ"),
		('ﲞ', "ﺑﺨ"), ('ﲟ', "ﺑﻤ"), ('ﲠ', "ﺑﻬ"), ('ﲡ', "ﺗﺠ"), ('ﲢ', "ﺗﺤ"), ('ﲣ', "ﺗﺨ"), ('ﲤ', "ﺗﻤ"), ('ﲥ', "ﺗﻬ"), ('ﳥ', "ﺜﻤ"), ('ﲧ', "ﺟﺤ"),
		('ﲨ', "ﺟﻤ"), ('ﲩ', "ﺣﺠ"), ('ﲪ', "ﺣﻤ"), ('ﲫ', "ﺧﺠ"), ('ﲬ', "ﺧﻤ"), ('ﲭ', "ﺳﺠ"), ('ﲮ', "ﺳﺤ"), ('ﲯ', "ﺳﺨ"), ('ﲰ', "ﺳﻤ"), ('ﲱ', "ﺻﺤ"),
		('ﲲ', "ﺻﺨ"), ('ﲳ', "ﺻﻤ"), ('ﲴ', "ﺿﺠ"), ('ﲵ', "ﺿﺤ"), ('ﲶ', "ﺿﺨ"), ('ﲷ', "ﺿﻤ"), ('ﲸ', "ﻃﺤ"), ('ﲹ', "ﻇﻤ"), ('ﲺ', "ﻋﺠ"), ('ﲻ', "ﻋﻤ"),
		('ﲼ', "ﻏﺠ"), ('ﲽ', "ﻏﻤ"), ('ﲾ', "ﻓﺠ"), ('ﲿ', "ﻓﺤ"), ('ﳀ', "ﻓﺨ"), ('ﳁ', "ﻓﻤ"), ('ﳂ', "ﻗﺤ"), ('ﳃ', "ﻗﻤ"), ('ﳄ', "ﻛﺠ"), ('ﳅ', "ﻛﺤ"),
		('ﳆ', "ﻛﺨ"), ('ﳇ', "ﻛﻠ"), ('ﳈ', "ﻛﻤ"), ('ﳉ', "ﻟﺠ"), ('ﳊ', "ﻟﺤ"), ('ﳋ', "ﻟﺨ"), ('ﳌ', "ﻟﻤ"), ('ﳍ', "ﻟﻬ"), ('ﳎ', "ﻣﺠ"), ('ﳏ', "ﻣﺤ"),
		('ﳐ', "ﻣﺨ"), ('ﳑ', "ﻣﻤ"), ('ﳒ', "ﻧﺠ"), ('ﳓ', "ﻧﺤ"), ('ﳔ', "ﻧﺨ"), ('ﳕ', "ﻧﻤ"), ('ﳖ', "ﻧﻬ"), ('ﳗ', "ﻫﺠ"), ('ﳘ', "ﻫﻤ"), ('ﳚ', "ﻳﺠ"),
		('ﳛ', "ﻳﺤ"), ('ﳜ', "ﻳﺨ"), ('ﳝ', "ﻳﻤ"), ('ﳞ', "ﻳﻬ"), ('ﳟ', "ﺌﻤ"), ('ﳠ', "ﺌﻬ"), ('ﳡ', "ﺒﻤ"), ('ﳢ', "ﺒﻬ"), ('ﳣ', "ﺘﻤ"), ('ﳤ', "ﺘﻬ"),
		('ﳦ', "ﺜﻬ"), ('ﳧ', "ﺴﻤ"), ('ﳨ', "ﺴﻬ"), ('ﳩ', "ﺸﻤ"), ('ﳪ', "ﺸﻬ"), ('ﳫ', "ﻜﻠ"), ('ﳬ', "ﻜﻤ"), ('ﳭ', "ﻠﻤ"), ('ﳮ', "ﻨﻤ"), ('ﳯ', "ﻨﻬ"),
		('ﳰ', "ﻴﻤ"), ('ﳱ', "ﻴﻬ"), ('ﳵ', "ﻃﻰ"), ('ﳶ', "ﻃﻲ"), ('ﳷ', "ﻋﻰ"), ('ﳸ', "ﻋﻲ"), ('ﳹ', "ﻏﻰ"), ('ﳺ', "ﻏﻲ"), ('ﳻ', "ﺳﻰ"), ('ﳼ', "ﺳﻲ"),
		('ﳽ', "ﺷﻰ"), ('ﳾ', "ﺷﻲ"), ('ﳿ', "ﺣﻰ"), ('ﴀ', "ﺣﻲ"), ('ﴁ', "ﺟﻰ"), ('ﴂ', "ﺟﻲ"), ('ﴃ', "ﺧﻰ"), ('ﴄ', "ﺧﻲ"), ('ﴅ', "ﺻﻰ"), ('ﴆ', "ﺻﻲ"),
		('ﴇ', "ﺿﻰ"), ('ﴈ', "ﺿﻲ"), ('ﴉ', "ﺷﺞ"), ('ﴊ', "ﺷﺢ"), ('ﴋ', "ﺷﺦ"), ('ﴌ', "ﺷﻢ"), ('ﴍ', "ﺷﺮ"), ('ﴎ', "ﺳﺮ"), ('ﴏ', "ﺻﺮ"), ('ﴐ', "ﺿﺮ"),
		('ﴑ', "ﻄﻰ"), ('ﴒ', "ﻄﻲ"), ('ﴓ', "ﻌﻰ"), ('ﴔ', "ﻌﻲ"), ('ﴕ', "ﻐﻰ"), ('ﴖ', "ﻐﻲ"), ('ﴗ', "ﺴﻰ"), ('ﴘ', "ﺴﻲ"), ('ﴙ', "ﺸﻰ"), ('ﴚ', "ﺸﻲ"),
		('ﴛ', "ﺤﻰ"), ('ﴜ', "ﺤﻲ"), ('ﴝ', "ﺠﻰ"), ('ﴞ', "ﺠﻲ"), ('ﴟ', "ﺨﻰ"), ('ﴠ', "ﺨﻲ"), ('ﴡ', "ﺼﻰ"), ('ﴢ', "ﺼﻲ"), ('ﴣ', "ﻀﻰ"), ('ﴤ', "ﻀﻲ"),
		('ﴥ', "ﺸﺞ"), ('ﴦ', "ﺸﺢ"), ('ﴧ', "ﺸﺦ"), ('ﴨ', "ﺸﻢ"), ('ﴩ', "ﺸﺮ"), ('ﴪ', "ﺴﺮ"), ('ﴫ', "ﺼﺮ"), ('ﴬ', "ﻀﺮ"), ('ﴭ', "ﺷﺠ"), ('ﴮ', "ﺷﺤ"),
		('ﴯ', "ﺷﺨ"), ('ﴰ', "ﺷﻤ"), ('ﴱ', "ﺳﻬ"), ('ﴲ', "ﺷﻬ"), ('ﴳ', "ﻃﻤ"), ('ﴴ', "ﺴﺠ"), ('ﴵ', "ﺴﺤ"), ('ﴶ', "ﺴﺨ"), ('ﴷ', "ﺸﺠ"), ('ﴸ', "ﺸﺤ"),
		('ﴹ', "ﺸﺨ"), ('ﴺ', "ﻄﻤ"), ('ﴻ', "ﻈﻤ"), ('ﴼ', "ﺎﱟ"), ('ﴽ', "ﺍﱟ"), ('ﵐ', "ﺗﺠﻤ"), ('ﵑ', "ﺘﺤﺞ"), ('ﵒ', "ﺗﺤﺠ"), ('ﵓ', "ﺗﺤﻤ"), ('ﵔ', "ﺗﺨﻤ"),
		('ﵕ', "ﺗﻤﺠ"), ('ﵖ', "ﺗﻤﺤ"), ('ﵗ', "ﺗﻤﺨ"), ('ﵘ', "ﺠﻤﺢ"), ('ﵙ', "ﺟﻤﺤ"), ('ﵚ', "ﺤﻤﻲ"), ('ﵛ', "ﺤﻤﻰ"), ('ﵜ', "ﺳﺤﺠ"), ('ﵝ', "ﺳﺠﺤ"),
		('ﵞ', "ﺴﺠﻰ"), ('ﵟ', "ﺴﻤﺢ"), ('ﵠ', "ﺳﻤﺤ"), ('ﵡ', "ﺳﻤﺠ"), ('ﵢ', "ﺴﻤﻢ"), ('ﵣ', "ﺳﻤﻤ"), ('ﵤ', "ﺼﺤﺢ"), ('ﵥ', "ﺻﺤﺤ"), ('ﵦ', "ﺼﻤﻢ"),
		('ﵧ', "ﺸﺤﻢ"), ('ﵨ', "ﺷﺤﻤ"), ('ﵩ', "ﺸﺠﻲ"), ('ﵪ', "ﺸﻤﺦ"), ('ﵫ', "ﺷﻤﺨ"), ('ﵬ', "ﺸﻤﻢ"), ('ﵭ', "ﺷﻤﻤ"), ('ﵮ', "ﻀﺤﻰ"), ('ﵯ', "ﻀﺨﻢ"),
		('ﵰ', "ﺿﺨﻤ"), ('ﵱ', "ﻄﻤﺢ"), ('ﵲ', "ﻃﻤﺤ"), ('ﵳ', "ﻃﻤﻤ"), ('ﵴ', "ﻄﻤﻲ"), ('ﵵ', "ﻌﺠﻢ"), ('ﵶ', "ﻌﻤﻢ"), ('ﵷ', "ﻋﻤﻤ"), ('ﵸ', "ﻌﻤﻰ"),
		('ﵹ', "ﻐﻤﻢ"), ('ﵺ', "ﻐﻤﻲ"), ('ﵻ', "ﻐﻤﻰ"), ('ﵼ', "ﻔﺨﻢ"), ('ﵽ', "ﻓﺨﻤ"), ('ﵾ', "ﻘﻤﺢ"), ('ﵿ', "ﻘﻤﻢ"), ('ﶀ', "ﻠﺤﻢ"), ('ﶁ', "ﻠﺤﻲ"),
		('ﶂ', "ﻠﺤﻰ"), ('ﶃ', "ﻟﺠﺠ"), ('ﶄ', "ﻠﺠﺞ"), ('ﶀ', "ﻠﺨﻢ"), ('ﶆ', "ﻟﺨﻤ"), ('ﶇ', "ﻠﻤﺢ"), ('ﶈ', "ﻟﻤﺤ"), ('ﶉ', "ﻣﺤﺠ"), ('ﶊ', "ﻣﺤﻤ"),
		('ﶋ', "ﻤﺤﻲ"), ('ﶌ', "ﻣﺠﺤ"), ('ﶍ', "ﻣﺠﻤ"), ('ﶎ', "ﻣﺨﺠ"), ('ﶏ', "ﻣﺨﻤ"), ('ﶒ', "ﻣﺠﺨ"), ('ﶓ', "ﻫﻤﺠ"), ('ﶔ', "ﻫﻤﻤ"), ('ﶕ', "ﻧﺤﻤ"),
		('ﶖ', "ﻨﺤﻰ"), ('ﶗ', "ﻨﺤﻢ"), ('ﶘ', "ﻧﺠﻤ"), ('ﶙ', "ﻨﺠﻰ"), ('ﶚ', "ﻨﻤﻲ"), ('ﶛ', "ﻨﻤﻰ"), ('ﶜ', "ﻴﻤﻢ"), ('ﶝ', "ﻳﻤﻤ"), ('ﶞ', "ﺒﺨﻲ"),
		('ﶟ', "ﺘﺠﻲ"), ('ﶠ', "ﺘﺠﻰ"), ('ﶡ', "ﺘﺨﻲ"), ('ﶢ', "ﺘﺨﻰ"), ('ﶣ', "ﺘﻤﻲ"), ('ﶤ', "ﺘﻤﻰ"), ('ﶥ', "ﺠﻤﻲ"), ('ﶦ', "ﺠﺤﻰ"), ('ﶧ', "ﺠﻤﻰ"),
		('ﶨ', "ﺴﺨﻰ"), ('ﶩ', "ﺼﺤﻲ"), ('ﶪ', "ﺸﺤﻲ"), ('ﶫ', "ﻀﺤﻲ"), ('ﶬ', "ﻠﺠﻲ"), ('ﶭ', "ﻠﻤﻲ"), ('ﶮ', "ﻴﺤﻲ"), ('ﶯ', "ﻴﺠﻲ"), ('ﶰ', "ﻴﻤﻲ"),
		('ﶱ', "ﻤﻤﻲ"), ('ﶲ', "ﻘﻤﻲ"), ('ﶳ', "ﻨﺤﻲ"), ('ﶴ', "ﻗﻤﺤ"), ('ﶵ', "ﻟﺤﻤ"), ('ﶶ', "ﻌﻤﻲ"), ('ﶷ', "ﻜﻤﻲ"), ('ﶸ', "ﻧﺠﺤ"), ('ﶹ', "ﻤﺨﻲ"),
		('ﶺ', "ﻟﺠﻤ"), ('ﶻ', "ﻜﻤﻢ"), ('ﶼ', "ﻠﺠﻢ"), ('ﶽ', "ﻨﺠﺢ"), ('ﶾ', "ﺠﺤﻲ"), ('ﶿ', "ﺤﺠﻲ"), ('ﷀ', "ﻤﺠﻲ"), ('ﷁ', "ﻔﻤﻲ"), ('ﷂ', "ﺒﺤﻲ"),
		('ﷃ', "ﻛﻤﻤ"), ('ﷄ', "ﻋﺠﻤ"), ('ﷅ', "ﺻﻤﻤ"), ('ﷆ', "ﺴﺨﻲ"), ('ﷇ', "ﻨﺠﻲ"), ('ﷲ', "ﺍﻟﻠﻪ"), ('ﷳ', "ﺍﻛﺒﺮ"), ('ﷴ', "ﻣﺤﻤﺪ"), ('ﷵ', "ﺻﻠﻌﻢ"),
		('ﷶ', "ﺭﺳﻮﻝ"), ('ﷺ', "ﺻﻠﻰ ﺍﻟﻠﻪ ﻋﻠﻴﻪ ﻭﺳﻠﻢ"), ('ﷷ', "ﻋﻠﻴﻪ"), ('ﷸ', "ﻭﺳﻠﻢ"), ('ﷹ', "ﺻﻠﻰ"), ('ﷻ', "ﺟﻞ ﺟﻼﻟﻪ"), ('﷼', "ﺭﻳﺎﻝ"),
		('﷽', "ﺑﺴﻢ ﺍﻟﻠﻪ ﺍﻟﺮﺣﻤﻦ ﺍﻟﺮﺣﻴﻢ")
	].iter().cloned().collect();
}

const ALL_HARAKAT: &str = concatcp!(HARAKAT, FREEZED_HARAKAT);
// const FULL_ARABIC: &str = concatcp!(ARABIC_CHARS, ARABIC_SYMBOLS, FREEZED_ARABIC_CHARS, ALL_HARAKAT);
// , MERGED_ARABIC_MAP.keys().map(|key| key.to_string()).collect::<Vec<String>>().join(""), BRACKETS_LIST.join(""));
const NACHURAL_CHARS: &str = concatcp!(RETURNS, SYMBOLS);
const CHARS_CONNECT: &str = concatcp!(CHARS_CONNECT_BOTH, CHARS_CONNECT_BEFORE);
const HARAKAT_PATTERN: &str = concatcp!("[", ALL_HARAKAT, "]");

// ======================= Useful Functions

pub fn get_subs(text: &str, sublen: usize) -> Vec<String> {
    if sublen > text.len() {
        return Vec::new();
    }
    if sublen > 0 {
		return text.chars().collect::<Vec<char>>().windows(sublen).map(|w| w.iter().collect()).collect();
    }
	(1..=text.chars().count()).rev().map(|sublen|
		text.chars().collect::<Vec<char>>().windows(sublen).map(|w| w.iter().collect::<String>()).collect::<Vec<String>>()
	).collect::<Vec<Vec<String>>>().concat()
}

pub fn add_every_x(text: &str, to_add: &str, every: usize) -> String {
    let text_len = text.chars().count();
	if every == 0 || every > text_len{
        return String::from(text);
    }
	let text: String = text.to_string();
	((0..text_len).step_by(every)).map(|x| text.chars().skip(x).take(every).collect::<String>() + to_add).collect()
}

pub fn devide_and_put_in(text: &str, to_add_into: &str, replace_with: &str, every: usize) -> String {
	let text: String = text.to_string();
	((0..text.chars().count()).step_by(every)).map(|x| to_add_into.replace(replace_with, &text.chars().skip(x).take(every).collect::<String>())).collect()
}

pub fn get_sorted_iterator(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut hash_vec: Vec<(&String, &String)> = map.iter().collect();
    hash_vec.sort_by(|a, b| b.0.chars().count().cmp(&a.0.chars().count()));
	hash_vec.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

pub fn swap_keys_and_values(map: &HashMap<String, String>) -> HashMap<String, String> {
	let mut inverted_map: HashMap<String, String> = HashMap::new();
	for (key, value) in map {
		inverted_map.insert(value.clone(), key.clone());
	}
	inverted_map
}

pub fn fix_for_regex(text: &str) -> String {
	regex::Regex::new(r"([\[\]|\\{}()+*^$?\.])").unwrap().replace_all(text, "\\$1").into_owned()
}

pub fn get_line_width(text: &str, fontmap: &HashMap<char, [isize; 8]>, coms_pattern: &str, offset_com: &str) -> isize {
    let offset_pattern = fix_for_regex(offset_com).replace("<offset>", "(-?\\d+)");
	let offset_regex = regex::Regex::new(&offset_pattern);
	let mut width: isize = match offset_regex {
        Ok(ref re) => re.captures_iter(&text).map(
				|x| x[1].parse::<isize>().unwrap()
			).sum::<isize>(),
        Err(_) => 0,
    };
	let text = match offset_regex {
        Ok(re) => re.replace_all(&text, "").to_string(),
        Err(_) => text.to_string(),
    };
	let text = match regex::Regex::new(coms_pattern) {
        Ok(ref re) => re.replace_all(&text, "").to_string(),
        Err(_) => text,
    };
	width += text.chars().map(
		|x|
		if fontmap.contains_key(&x) {
			fontmap[&x][2] + fontmap[&x][6]
		} else {
			0
		}
	).sum::<isize>();
    width
}

pub fn get_dir_files_paths(dir_path: &str) -> Vec<String> {
	match fs::read_dir(dir_path) {
		Ok(f) => f.filter_map(|entry| {
			let path = entry.unwrap().path();
			if path.is_file() {
				Some(path.display().to_string())
			} else {
				None
			}
		}).collect(),
		Err(_) => return Vec::new(),
	}
}

pub fn get_dir_files_tree(dir_path: &str) -> Vec<String> {
	let dir = match fs::read_dir(dir_path) {
		Ok(f) => f,
		Err(_) => return Vec::new(),
	};
	let mut tree: Vec<String> = Vec::new();
	for entry in dir {
		let path = entry.unwrap().path();
		if path.is_file() {
			tree.push(path.display().to_string());
		} else if path.is_dir() {
			tree.extend(get_dir_files_tree(path.to_str().unwrap()));
		}
	}
	tree
}

pub fn ends_with_any(string: &str, suffixes: &[&str]) -> bool {
	suffixes.iter().any(|&suffix| string.ends_with(suffix))
}

pub fn open_and_read_csv(path: &str) -> Vec<Vec<String>> {
    let reader: BufReader<fs::File>;
	match fs::File::open(path) {
		Ok(f) => reader = BufReader::new(f),
		Err(_) => return Vec::new(),
	}

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(CSV_DELIMITER as u8)
        .has_headers(false)
        .flexible(true)
        .from_reader(reader);

	let csv = Some(csv_reader.records().map(|row| row.unwrap().iter().map(|f| f.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>());
	csv.unwrap_or(Vec::new())
}

pub fn split_keep(text: &str, pattern: &str) -> Vec<String> {
	if text.is_empty() {
        return Vec::new();
    }
	let mut result = Vec::new();
	let mut last = 0;
	match regex::Regex::new(pattern) {
        Ok(re) => 
			for m in re.find_iter(text) {
				result.push(text[last..m.start()].to_string());
				result.push(m.as_str().to_string());
				last = m.end();
			},
        Err(_) => (),
    };
    result.push(text[last..].to_string());
    result
}

pub fn while_starts_with_remove(text: &str, suffix: &str) -> String {
	let mut text = text;
	while text.starts_with(suffix) && !suffix.is_empty() {
		text = &text[suffix.len()..]
	}
	text.to_string()
}

pub fn copy_to_clipboard(string: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.to_owned()).unwrap();
}

pub fn try_write_byte_file(path: &str, content: &[u8]) {
    match fs::File::create(path) {
		Ok(mut f) => f.write_all(content),
		Err(_) => Ok(println!("Failed to create file ({}). Access denied?", path)),
	};
}

pub fn try_write_string_file(path: &str, content: &str) {
	try_write_byte_file(path, content.as_bytes());
}

pub fn repeat_char(c: char, n: usize) -> String {
    std::iter::repeat(c).take(n).collect()
}

pub fn fix_escape_chars(string: &str) -> String {
	let string = string.to_string();
	let mut new_string = String::new();
	let mut i: usize = 0;
	while i < string.chars().count() {
		let c = string.chars().nth(i).unwrap();
		if c == '\\' {
			if let Some(nc) = string.chars().nth(i+1) {
				new_string.push(match nc {
					'n' => {i += 1; '\n'},
					'r' => {i += 1; '\r'},
					't' => {i += 1; '\t'},
					'0' => {i += 1; '\0'},
					'\\' => {i += 1; '\\'},
					_ => nc
				});
			} else {
				new_string.push(c);
			}
		} else {
			new_string.push(c);
		}
		i += 1;
	}
	new_string
}

// ======================= Freeze Text

pub fn freeze(text: &str) -> String {
    text.chars().enumerate().map(|(i, c)|
        if FREEZED_ARABIC_MAP.contains_key(&c) {
			FREEZED_ARABIC_MAP[&c][
				(CHARS_CONNECT_BOTH.contains(
					text.chars().take(i).collect::<Vec<char>>().into_iter().rev().find(|&chr| !ALL_HARAKAT.contains(chr)).unwrap_or(' ')
				) && CHARS_CONNECT.contains(c)) as usize + 2 * (CHARS_CONNECT.contains(
					text.chars().skip(i+1).find(|&chr| !ALL_HARAKAT.contains(chr)).unwrap_or(' ')
				)) as usize
			]
        } else {
            c
		}
	).collect()
}

pub fn unfreeze(text: &str) -> String {
    let mut text: String = text.to_owned();
    for (key, value) in FREEZED_ARABIC_MAP.iter() {
		for state in value {
			text = text.replace(*state, &key.to_string());
		}
	}
    text
}

// ======================= Extract From Text

// pub fn extract_from_text<'a>(text: &'a str, pattern: &'a str, line_com: &'a str) -> String {
// 	regex::Regex::new(pattern).unwrap_or(regex::Regex::new("").unwrap()).captures_iter(text).filter_map(|x| x.get(1)).map(|m| m.as_str()).collect::<Vec<&str>>().join(line_com)
// }

// ======================= Compress Text

pub fn compress_text(text: &str, used_ligatures: &str) -> String {
    let mut text: String = text.to_string();
	if text.is_empty() || used_ligatures.is_empty() {
		return text;
	}
	
	for ligature in used_ligatures.chars() {
		let value = MERGED_ARABIC_MAP.get(&ligature);
		if !value.is_some() {
			continue;
		}
		text = text.replace(value.unwrap(), &ligature.to_string());
	}
	text
}

pub fn uncompress_text(text: &str) -> String { // used_ligatures
    let mut text: String = text.to_string();
	for (k, v) in MERGED_ARABIC_MAP.iter() {
		text = text.replace(*k, v);
	}
	text
}

pub fn suggest_ligatures(text: &str, ligature_lens: &Vec<usize>, results_num: usize) -> Vec<String> {
	let mut ligatures_vec: Vec<String> = Vec::new();
	for ligaturelen in ligature_lens.into_iter() {
		ligatures_vec.extend(get_subs(text, *ligaturelen));
	}
	ligatures_vec.concat();
	ligatures_vec.sort_by(|a, b| text.matches(b).count().cmp(&text.matches(a).count()));
	ligatures_vec.dedup();
	(&ligatures_vec[..cmp::min(results_num, ligatures_vec.len())]).to_vec()
}

// ======================= Reverse Text

pub fn reverse_text(text: &str) -> String {
	text.chars().rev().collect()
}

pub fn reverse_brackets(text: &str) -> String {
	let mut text: String = text.to_string();
	for bracket in BRACKETS_LIST {
		let b1: char = bracket.chars().nth(0).unwrap();
		let b2: char = bracket.chars().nth(1).unwrap();
		text = text.replace(b1, "\0").replace(b2, &b1.to_string()).replace("\0", &b2.to_string());
	}
	text
}

pub fn reverse_arabic(text: &str) -> String {
	let FULL_ARABIC: &str = &(ARABIC_CHARS.to_owned() + ARABIC_SYMBOLS + FREEZED_ARABIC_CHARS + ALL_HARAKAT + &MERGED_ARABIC_MAP.keys().map(|key| key.to_string()).collect::<Vec<String>>().join("") + &BRACKETS_LIST.join(""));
	let mut last_pos: usize = 0;
	let mut newtext: Vec<String> = Vec::new();
	for x in regex::Regex::new(
			&(
			"([".to_owned() + &fix_for_regex(FULL_ARABIC) + &fix_for_regex(NACHURAL_CHARS) + "]*"
			+ "[" + &fix_for_regex(FULL_ARABIC) + "]"
			+ "[" + &fix_for_regex(FULL_ARABIC) + &fix_for_regex(NACHURAL_CHARS) + "]*)"
			+ "|([" + &fix_for_regex(NACHURAL_CHARS) + "]*$)"
			)
		).unwrap().captures_iter(text) {
		newtext.push(reverse_text(x.get(0).unwrap().as_str()) + &text[last_pos..x.get(0).unwrap().start()]);
		last_pos = x.get(0).unwrap().end();
	}
	newtext.push(text[last_pos..].to_string());

    newtext.reverse();
    reverse_brackets(&newtext.join(""))
}

// ======================= Convert Harakat

pub fn del_harakat(text: &str) -> String {
	let mut text: String = text.to_string();
	for h in ALL_HARAKAT.chars() {
		text = text.replace(h, "");
	}
	text
}

pub fn keep_first_haraka(text: &str) -> String {
	let mut newtext: String = text.chars().next().unwrap().to_string();
	for (i, c) in text.chars().enumerate().skip(1) {
		if !(ALL_HARAKAT.contains(text.chars().nth(i-1).unwrap()) && ALL_HARAKAT.contains(c)) {
			newtext.push(c);
		}
	}
	newtext
}

pub fn merge_harakat(text: &str) -> String {
	let mut text: String = text.to_string();
	for (k, v) in MERGED_HARAKAT_MAP.iter() {
		for comb in (0..v.len()).step_by(2) {
			for a in v[comb+0].chars() {
				for b in v[comb+1].chars() {
					text = text.replace(&format!("{}{}", a, b), *k);
				}
			}
		}
	}
	text
}

pub fn offset_harakat(text: &str, offset: isize) -> String {
	let mut newtext: String = del_harakat(text);
	for (i, c) in text.chars().enumerate() {
		if !ALL_HARAKAT.contains(c) {
			continue;
		}
		let index: usize = cmp::min(cmp::max((i as isize)+offset, 0) as usize, newtext.len()) as usize;
		newtext = newtext.chars().take(index).collect::<String>() + &c.to_string() + &newtext.chars().skip(index).collect::<String>();
	}
	newtext
}

pub fn connect_harakat(text: &str) -> String {
    text.chars().enumerate().map(|(i, c)|
        if ALL_HARAKAT.contains(c) && CHARS_CONNECT_BOTH.contains (
				text.chars().take(i).collect::<Vec<char>>().into_iter().rev().find(|&chr| !ALL_HARAKAT.contains(chr)).unwrap_or(' ')
			) && CHARS_CONNECT.contains (
				text.chars().skip(i+1).find(|&chr| !ALL_HARAKAT.contains(chr)).unwrap_or(' ')
			) {
			CONNECTED_HARAKAT_MAP[&c]
        } else {
            c
		}
	).collect()
}

// ======================= Convert Text-Hex-Unicode

pub fn encode_hex(text: &str) -> String {
	hex::encode(text)
}

pub fn decode_hex(hex_string: &str) -> String {
	let bytes: Vec<u8> = hex::decode(hex_string).unwrap_or(Vec::new());
	String::from_utf8(bytes).unwrap_or("".to_string())
}

pub fn encode_unicode(text: &str) -> String {
	let mut hex_string: String = String::new();
	let mut h: String;
	for c in text.chars() {
		h = format!("{:X}", c as u32);
		hex_string += &"0".repeat(4-h.len());
		hex_string += &h;
	}
	hex_string
}

pub fn decode_unicode(text: &str) -> String {
	(0..text.len()).step_by(4).map(
		|x| std::char::from_u32(u32::from_str_radix(&text[x..x+4], 16).unwrap_or(0)).unwrap()
	).collect()
}

pub fn text_to_hex(text: &str, outbyte_form: &str, bytes: usize) -> String {
	let l: usize = bytes * 2;
	text.chars().map(
		|x| encode_hex(&x.to_string())
	).map(
		|y| devide_and_put_in(
			&("0".repeat(((l as isize - y.len() as isize) * (l > y.len()) as isize) as usize) + &y),
			outbyte_form, "<byte>", l
		)
	).collect()
}

pub fn hex_to_text(text: &str, inbyte_pattern: &str) -> String {
	regex::Regex::new(inbyte_pattern).unwrap().captures_iter(text).map(
		|x| decode_hex(&while_starts_with_remove(x.get(1).unwrap().as_str(), "00"))
	).collect()
}

pub fn text_to_unicode(text: &str, outunicode_form: &str) -> String { // , bytes: usize
	// let l: usize = bytes * 2;
	text.chars().map(
		|x| encode_unicode(&x.to_string())
	).map(
		|y| devide_and_put_in(
			&("0".repeat(((4 - y.len() as isize) * (4 > y.len()) as isize) as usize) + &y),
			outunicode_form, "<unicode>", 4
		)
	).collect()
}

pub fn unicode_to_text(text: &str, inunicode_pattern: &str) -> String {
	regex::Regex::new(inunicode_pattern).unwrap().captures_iter(text).map(
		|x| decode_unicode(x.get(1).unwrap().as_str())
	).collect()
}

// ======================= Load Tables

pub fn load_act(content: &str) -> HashMap<String, String> {
	let mut charmap: HashMap<String, String> = HashMap::new();
	for l in content.replace("\r\n", "\n").split("\n").map(
			|x| x.split(A_SEPARATOR).collect::<Vec<&str>>()
		).collect::<Vec<Vec<&str>>>().iter().skip(1) {
		if l[0].len() >= 1 {
			charmap.extend(char_to_char(&l[0], [&l.get(1).unwrap_or(&""), &l.get(2).unwrap_or(&""), &l.get(3).unwrap_or(&""), &l.get(4).unwrap_or(&"")]));
		}
	}
	charmap
}

pub fn load_tbl(content: &str) -> HashMap<String, String> {
	let charmap: HashMap<String, String> = regex::Regex::new("([a-zA-Z0-9]*)=(\\S+)").unwrap().captures_iter(content).map(
		|x| (x[2].to_string(), decode_hex(&x[1]))
	).collect();
	charmap
}

pub fn load_zts(content: &str) -> HashMap<String, String> { // zip two strings
	let mut charmap: HashMap<String, String> = HashMap::new();
	let lines: Vec<Vec<char>> = content.replace("\r\n", "\n").split("\n").map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();
	if lines.len() >= 2 {
		for i in 0..cmp::min(lines[0].len(), lines[1].len()) {
			charmap.insert(lines[1][i].to_string(), lines[0][i].to_string());
		}
	}
	charmap
}

pub fn load_zta(content: &str) -> HashMap<String, String> {
	let content = content.to_string();
	let mut cont = String::new();
	let mut last_pos: usize = 0;
	if content.len() >= 5 {
		for (i, c) in content.chars().take(content.chars().count()-4).enumerate() {
			if c == '{' && content.chars().nth(i+2).unwrap() == ':' && content.chars().nth(i+4).unwrap() == '}' { // range: {_:_}
				cont += &content.chars().skip(last_pos).take(i-last_pos).collect::<String>();
				cont += &(content.chars().nth(i+1).unwrap()..=content.chars().nth(i+3).unwrap()).collect::<String>();
				last_pos = i+5;
			}
		}
	}
	cont += &&content.chars().skip(last_pos).collect::<String>();
	load_zts(&cont)
}

pub fn load_ciphering_table(path: &str, complete_ar: bool) -> HashMap<String, String> {
	let mut charmap: HashMap<String, String> = HashMap::new();
	let content: String = fs::read_to_string(path).unwrap_or(String::new());
	if content.is_empty() {
		return charmap;
	}
	if path.ends_with(".act") {
		charmap = load_act(&content);
	}
	else if path.ends_with(".tbl") {
		charmap = load_tbl(&content);
	}
	else if path.ends_with(".zts") {
		charmap = load_zts(&content);
	}
	else if path.ends_with(".zta") {
		charmap = load_zta(&content);
	}
	else {
		return charmap;
	}
	if complete_ar {
		return complete_arabic_cipher(charmap);
	}
	charmap
}

pub fn load_fnt(content: &str) -> HashMap<char, [isize; 8]> {
	let mut find: &str;
	if content.starts_with("<?xml version=\"1.0\"?>") { //Xml
		find = "<char id=\"(.*?)\" x=\"(.*?)\" y=\"(.*?)\" width=\"(.*?)\" height=\"(.*?)\" xoffset=\"(.*?)\" yoffset=\"(.*?)\" xadvance=\"(.*?)\" page=\"(.*?)\""; //  chnl=\"(.*?)\" />
		// find = ["<page id=\"(.*?)\" file=\"(.*?)\" />", "<char id=\"(.*?)\" x=\"(.*?)\" y=\"(.*?)\" width=\"(.*?)\" height=\"(.*?)\" xoffset=\"(.*?)\" yoffset=\"(.*?)\" xadvance=\"(.*?)\" page=\"(.*?)\""]; //  chnl=\"(.*?)\" />
	} else { //Text
		find = "char id=(-?\\d+) *?x=(-?\\d+) *?y=(-?\\d+) *?width=(-?\\d+) *?height=(-?\\d+) *?xoffset=(-?\\d+) *?yoffset=(-?\\d+) *?xadvance=(-?\\d+) *?page=(-?\\d+)"; //  *?chnl=(-?\\d+)
		// find = ["page id=(\\d+) file=\"(.*?)\"", "char id=(.*?)[ *?]x=(.*?)[ *?]y=(.*?)[ *?]width=(.*?)[ *?]height=(.*?)[ *?]xoffset=(.*?)[ *?]yoffset=(.*?)[ *?]xadvance=(.*?)[ *?]page=(.*?)"]; // [ *?]chnl=(.*?)
	}
	
	let mut charmap: HashMap<char, [isize; 8]> = HashMap::new();
	let mut tallest: isize = 0;
	for row in regex::Regex::new(find).unwrap().captures_iter(content) {
		charmap.insert(std::char::from_u32(row[1].parse::<u32>().unwrap()).unwrap(), [row[2].parse::<isize>().unwrap(), row[3].parse::<isize>().unwrap(), row[4].parse::<isize>().unwrap(), row[5].parse::<isize>().unwrap(), row[6].parse::<isize>().unwrap(), row[7].parse::<isize>().unwrap(), row[8].parse::<isize>().unwrap(), row[9].parse::<isize>().unwrap()]);
		if row[5].parse::<isize>().unwrap() > tallest {
			tallest = row[5].parse().unwrap();
		}
	}
	charmap.insert('\0', [tallest, 0, 0, 0, 0, 0, 0, 0]);
	charmap
}

/* pub fn load_AFF(content) {
  const table = content.split(Returns.subs().reverse().join('|').toregex::Regex('g')).map(x => x.split(_A_SEPARATOR_));
  
  var charmap = {tallest: 0, scale: 1, type: "aff", pages: {}};
  for (var r of range(1, table.length)) {
    if (!table[r].length) {continue}
    const drowData = table[r][1].split(_AFF_MIN_SEPARATOR);
    
    width = 0;
    for (var slice of drowData) {if (slice.length > width) {width = slice.length}}
    charmap[table[r][0]] = {x: 0, y: 0, w: width, h: drowData.length, xoff: parseInt(table[r][2]), yoff: parseInt(table[r][3]), xadv: parseInt(table[r][4]), drawData: drowData};
    if (drowData.length > charmap.tallest) {charmap.tallest = drowData.length}
  }
  return charmap;
} */

pub fn load_font_table(path: &str, complete_ar: bool) -> HashMap<char, [isize; 8]> {
	let mut charmap: HashMap<char, [isize; 8]> = HashMap::new();
	let content: String = fs::read_to_string(path).unwrap_or(String::new());
	if content.is_empty() {
		return charmap;
	}
	if path.ends_with(".fnt") {
		charmap = load_fnt(&content);
	}
	// else if path.ends_with('.ttf') {
		// charmap = load_TTF(&content, chars, fontSize)
	// }
	// else if path.ends_with(".aff") {
		// charmap = load_AFF(&content);
	// }
	else {
		return charmap;
	}
	if complete_ar {
		return complete_arabic_font(charmap);
	}
	charmap
}

// ======================= Convert Tables

pub fn complete_arabic_cipher(charmap: HashMap<String, String>) -> HashMap<String, String> {
	let mut charmap: HashMap<String, String> = charmap;
    for v in FREEZED_ARABIC_MAP.values() {
        let mut v0: bool = charmap.contains_key(&v[0].to_string());
        let mut v1: bool = charmap.contains_key(&v[1].to_string());
        let mut v2: bool = charmap.contains_key(&v[2].to_string());
        let mut v3: bool = charmap.contains_key(&v[3].to_string());
        if (v0 && v1 && v2 && v3) || !(v0 || v1 || v2 || v3) {
            continue;
        }
        if !v1 && v0 {
			charmap.insert(v[1].to_string(), charmap[&v[0].to_string()].clone());
            v1 = true;
        }
        else if !v0 && v1 {
			charmap.insert(v[0].to_string(), charmap[&v[1].to_string()].clone());
            v0 = true;
        }
        if !v3 && v2 {
			charmap.insert(v[3].to_string(), charmap[&v[2].to_string()].clone());
            v3 = true;
        }
        else if !v2 && v3 {
			charmap.insert(v[2].to_string(), charmap[&v[3].to_string()].clone());
            v2 = true;
        }
        if !v1 && v2 {
			charmap.insert(v[1].to_string(), charmap[&v[2].to_string()].clone());
            v1 = true;
        }
        else if !v2 && v1 {
			charmap.insert(v[2].to_string(), charmap[&v[1].to_string()].clone());
        }
        if !v0 && v3 {
			charmap.insert(v[0].to_string(), charmap[&v[3].to_string()].clone());
            v0 = true;
        }
        else if !v3 && v0 {
			charmap.insert(v[3].to_string(), charmap[&v[0].to_string()].clone());
        }
    }
    charmap
}

pub fn complete_arabic_font(fontmap: HashMap<char, [isize; 8]>) -> HashMap<char, [isize; 8]> {
	let mut fontmap: HashMap<char, [isize; 8]> = fontmap;
    for v in FREEZED_ARABIC_MAP.values() {
        let mut v0: bool = fontmap.contains_key(&v[0]);
        let mut v1: bool = fontmap.contains_key(&v[1]);
        let mut v2: bool = fontmap.contains_key(&v[2]);
        let mut v3: bool = fontmap.contains_key(&v[3]);
        if (v0 && v1 && v2 && v3) || !(v0 || v1 || v2 || v3) {
            continue;
        }
        if !v1 && v0 {
			fontmap.insert(v[1], fontmap[&v[0]].clone());
            v1 = true;
        }
        else if !v0 && v1 {
			fontmap.insert(v[0], fontmap[&v[1]].clone());
            v0 = true;
        }
        if !v3 && v2 {
			fontmap.insert(v[3], fontmap[&v[2]].clone());
            v3 = true;
        }
        else if !v2 && v3 {
			fontmap.insert(v[2], fontmap[&v[3]].clone());
            v2 = true;
        }
        if !v1 && v2 {
			fontmap.insert(v[1], fontmap[&v[2]].clone());
            v1 = true;
        }
        else if !v2 && v1 {
			fontmap.insert(v[2], fontmap[&v[1]].clone());
        }
        if !v0 && v3 {
			fontmap.insert(v[0], fontmap[&v[3]].clone());
            v0 = true;
        }
        else if !v3 && v0 {
			fontmap.insert(v[3], fontmap[&v[0]].clone());
        }
    }
    fontmap
}

pub fn char_to_char(string: &str, char_stats: [&str; 4]) -> HashMap<String, String> {
	let mut ctc_map: HashMap<String, String> = HashMap::new();
	let c: char = string.chars().next().unwrap();
	
	if string.chars().count() == 1 && FREEZED_ARABIC_MAP.contains_key(&c) {
		let freezed_states: [char; 4] = FREEZED_ARABIC_MAP[&c];
		for i in 0..4 {
			if !(char_stats[i].is_empty() && ctc_map.contains_key(&freezed_states[i].to_string())) {
				ctc_map.insert(freezed_states[i].to_string(), char_stats[i].to_string());
			}
		}
	} else {
		ctc_map.insert(string.to_string(), char_stats[0].to_string());
	}
	ctc_map
}

pub fn one_to_four_charmap(charmap: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut charmap: HashMap<String, String> = charmap.clone();
    let mut new_charmap: HashMap<String, Vec<String>> = HashMap::new();
    for (key, value) in FREEZED_ARABIC_MAP.iter() {
        let mut new_value: Vec<String> = Vec::new();
        let mut pass: bool = true;
        for v in value {
			if new_value.contains(&v.to_string()) {
				continue;
			}
			if charmap.contains_key(&v.to_string()) {
				new_value.push(charmap[&v.to_string()].clone());
				charmap.remove(&v.to_string());
				pass = false;
			}
        }
        if pass {
            continue;
        }
        new_charmap.insert(key.to_string(), new_value);
    }
	new_charmap.extend(charmap.iter().map(
		|x| (x.0.to_string(), vec![x.1.to_string()])
	).collect::<HashMap<String, Vec<String>>>());
    return new_charmap;
}

// ======================= Cipher Text

pub fn cipher(text: &str, charmap: &HashMap<String, String>, flip: bool) -> String {
    let mut text: String = text.to_string();
	if charmap.is_empty() || text.is_empty() {
		return text;
	}
	let mut charmap: HashMap<String, String> = charmap.clone();
	if flip {
		charmap = swap_keys_and_values(&charmap);
		charmap.remove("");
	}
	for (key, value) in get_sorted_iterator(&charmap) {
		text = text.replace(&key, &value);
	}
	text
}

// ======================= Save Tables

pub fn save_act(charmap: &HashMap<String, String>) -> String {
    ACT_HEADER.to_string() + "\n" + &one_to_four_charmap(charmap).iter().map(|x| x.0.to_owned() + A_SEPARATOR + &x.1.join(A_SEPARATOR)).collect::<Vec<String>>().join("\n")
}

pub fn save_tbl(charmap: &HashMap<String, String>) -> String {
	charmap.iter().map(|x| encode_hex(&x.1) + "=" + &x.0).collect::<Vec<String>>().join("\n")
}

pub fn save_table(path: &str, charmap: &HashMap<String, String>) {  // only .act and .tbl because they support multi chars ciphering
	let charmap: HashMap<String, String> = charmap.clone();
	let mut content: String = String::new();
    if path.ends_with(".act") {
        content = save_act(&charmap);
    }
    else if path.ends_with(".tbl") {
        content = save_tbl(&charmap);
    }
    else {
        return;
    }
	try_write_string_file(path, &content);
}

// ======================= Offset Text

pub fn offset_line_with_spaces(line_text: &str, fontmap: &HashMap<char, [isize; 8]>, font_size: usize, line_width: usize, text_align: u8, coms_pattern: &str, offset_com: &str) -> String {
    if (fontmap.is_empty() || !fontmap.contains_key(&'\0') || !fontmap.contains_key(&' ') || ((fontmap[&' '][2] + fontmap[&' '][6]) <= 0))
		|| text_align == 5 && (!fontmap.contains_key(&'ـ') || ((fontmap[&'ـ'][2] + fontmap[&'ـ'][6]) <= 0)) {
		return line_text.to_string();
	}
	let scale: f32 = fontmap[&'\0'][0] as f32 / font_size as f32;
	let line_width: isize = (line_width as f32 * scale) as isize;
	let spaces_num: isize = (line_width - get_line_width(&line_text, &fontmap, coms_pattern, offset_com)) / (fontmap[&' '][2] + fontmap[&' '][6]);
    if spaces_num <= 0 {
        return line_text.to_string();
    }
    if text_align == 0 { // left
        return line_text.to_string() + &" ".repeat(spaces_num as usize);
    }
    if text_align == 1 { // right
        return " ".repeat(spaces_num as usize) + line_text;
    }
    if text_align == 2 { // middle from left
        return " ".repeat((spaces_num - (spaces_num / 2)) as usize) + line_text;
    }
    if text_align == 3 { // middle from right
        return line_text.to_string() + &" ".repeat((spaces_num / 2) as usize);
    }
    if text_align == 4 { // middle from both
        return " ".repeat((spaces_num - (spaces_num / 2)) as usize) + line_text + &" ".repeat((spaces_num / 2) as usize);
    }
    if text_align == 5 { // justified
		let mut line_text = line_text.to_string();
		let mut strechable_poses = line_text.chars().enumerate().filter_map(|(i, x)| 
			if x == ' ' || x == 'ـ' {
				Some(i)
			} else {
				None
			}
		).into_iter().collect::<Vec<usize>>();
		if strechable_poses.is_empty() {
			return line_text;
		}
		strechable_poses.reverse();
		
		let n = spaces_num as usize / strechable_poses.len();
		let mut left_space: usize = spaces_num as usize % strechable_poses.len();
		
		let chars = line_text.chars().collect::<Vec<char>>();
		let mut string_chars = chars.iter().map(|x| x.to_string()).collect::<Vec<String>>();
		
		for (i, p) in strechable_poses.into_iter().enumerate() {
			let m = n / (fontmap[&chars[p]][2] + fontmap[&chars[p]][6]) as usize + left_space;
			left_space = n % (fontmap[&chars[p]][2] + fontmap[&chars[p]][6]) as usize;
			string_chars.insert(p, string_chars[p].repeat(m));
		}
		return string_chars.join("");
    }
    line_text.to_string()
}

/* pub fn offset_line_with_commands(line_text: &str, fontmap: &HashMap<char, [isize; 8]>, font_size: usize, line_width: usize, text_align: u8, coms_pattern: &str, offset_com: &str) -> String {
	if fontmap.is_empty() || !fontmap.contains_key(&'\0') {
		return line_text.to_string();
	}
	let mut text;
	if text_align == 5 {
		text = line_text.replace(" ", "");
	} else {
		text = line_text.to_string();
	}
	let scale: f32 = font_size as f32 / fontmap[&'\0'][0] as f32;
	let free_space: isize = (line_width as f32 - get_line_width(&text, &fontmap, coms_pattern, offset_com) as f32 * scale) as isize;
    if free_space <= 0 {
        return line_text.to_string();
    }
    if text_align == 0 { del
        return line_text.to_string() + &offset_com.replace("<offset>", &free_space.to_string());
    }
    if text_align == 1 {
		return offset_com.replace("<offset>", &free_space.to_string()) + line_text;
    }
    if text_align == 2 {
        let a: usize = free_space as usize / 2;
        return offset_com.replace("<offset>", &a.to_string()).repeat((a > 0) as usize) + line_text;
    }
    if text_align == 3 {
        let a: usize = free_space as usize / 2;
        return line_text.to_string() + &offset_com.replace("<offset>", &(free_space as usize - a).to_string()).repeat((free_space as usize - a > 0) as usize);
    }
    if text_align == 4 { del
        let a: usize = free_space as usize / 2;
        return offset_com.replace("<offset>", &a.to_string()).repeat((a > 0) as usize)
			+ line_text
			+ &offset_com.replace("<offset>", &(free_space as usize - a).to_string()).repeat((free_space as usize - a > 0) as usize);
    }
    // if text_align == 5 { // justified
	// 	let mut line_text = line_text.to_string();
	// 	let mut strechable_poses = line_text.chars().enumerate().filter_map(|(i, x)| 
	// 		if x == ' ' || x == 'ـ' {
	// 			Some(i)
	// 		} else {
	// 			None
	// 		}
	// 	).into_iter().collect::<Vec<usize>>();
	// 	if strechable_poses.is_empty() {
	// 		return line_text;
	// 	}
	// 	strechable_poses.reverse();
	// 	let n = spaces_num as usize / strechable_poses.len() - 1;
	// 	let m = spaces_num as usize % strechable_poses.len();
	// 	let mut chars = line_text.chars().map(|x| x.to_string()).collect::<Vec<String>>();

	// 	for (i, p) in strechable_poses.into_iter().enumerate() {
	// 		// let freepx = n + (i < m) as usize;
	// 		// if chars[p] == " " {
	// 			// new_text += &offset_com.replace("<offset>", &(n + (i < m) as usize).to_string());
	// 		// }
	// 		// else {

	// 		// }
	// 		chars.insert(p, chars[p].repeat(n + (i < m) as usize));
	// 	}
	// 	return chars.join("");
    // }
    return line_text.to_string();
} */

// ======================= Warp Text

pub fn split_line(line_text: &str) -> Vec<String> {
	let mut line_vec: Vec<String> = Vec::new();
	let mut last_word: &str = "";
	for word in line_text.split(" ") {
		if NOT_LAST_IN_LINE.contains(&last_word) {
			let vec_len: usize = line_vec.len();
			line_vec[vec_len-1] += &(" ".to_owned() + word);
		} else {
			line_vec.push(word.to_string());
		}
		last_word = word;
	}
	line_vec
}

pub fn word_warp(text: &str, fontmap: &HashMap<char, [isize; 8]>, font_size: usize, box_size: [usize; 2], px_per_line: usize, page_com: &str, line_com: &str, coms_pattern: &str, offset_com: &str) -> String {
    if fontmap.is_empty(){
		return text.to_string();
	}
	let mut x: isize;
	let mut y: isize;
	let scale: f32 = fontmap[&'\0'][0] as f32 / font_size as f32;
	let px_per_line: isize = (px_per_line as f32 * scale) as isize;
	let box_size: [isize; 2] = [(box_size[0] as f32 * scale) as isize, (box_size[1] as f32 * scale) as isize]; // less than 1 pixel error range because of using ints

    let mut pages: Vec<String> = text.split(page_com).map(|x| x.to_string()).collect();
	for i in 0..pages.len() {
        y = fontmap[&'\0'][0];
		let mut lines: Vec<String> = pages[i].split(line_com).map(|x| x.to_string()).collect();
		for j in 0..lines.len() {
			x = 0;
			lines[j] = split_line(&lines[j]).iter().map(|word|
                if get_line_width(word, &fontmap, coms_pattern, offset_com) + x > box_size[0] {
					if x == 0 {
						x = get_line_width(&(word.to_owned() + " "), &fontmap, coms_pattern, offset_com);
						word.to_owned() + " "
					} else {
						y += fontmap[&'\0'][0] + px_per_line;
						x = get_line_width(&(word.to_owned() + " "), &fontmap, coms_pattern, offset_com);
						if y > box_size[1] {
							y = fontmap[&'\0'][0];
							page_com.to_owned() + word + " "
						} else {
							line_com.to_owned() + word + " "
						}
					}
				} else {
                    x += get_line_width(&(word.to_owned() + " "), &fontmap, coms_pattern, offset_com);
					word.to_owned() + " "
                }
			).collect();
			if j+1 < lines.len() {
				y += fontmap[&'\0'][0] + px_per_line;
				if y > box_size[1] {
					y = fontmap[&'\0'][0];
					lines[j] += page_com;
				} else {
					lines[j] += line_com;
				}
			} else {
				lines[j].pop();
			}
        }
        pages[i] = lines.join("");
    }
    pages.join(page_com).replace(&(" ".to_owned()+page_com), page_com).replace(&(" ".to_owned()+line_com), line_com)
}

// ======================= Extract-Enter Sheet

pub fn extract_sheet_from_file(extract_from_path: &str, pattern: &str) -> Vec<[String; 2]> {
    let mut bytes_vec: Vec<u8>;
	match fs::read(extract_from_path) {
		Ok(f) => bytes_vec = f,
		Err(_) => return Vec::new(),
	}
	let bytes: &[u8] = bytes_vec.as_slice();
	regex::bytes::Regex::new(pattern).unwrap().captures_iter(bytes).map(|found|
		[found.get(1).unwrap().start().to_string(), String::from_utf8_lossy(&found[1]).to_string()]
	).collect()
}

pub fn extract_sheet_from_folder(extract_from_path: &str, filetypes: &[&str], pattern: &str) -> Vec<[String; 2]> {
    let mut e: Vec<[String; 2]>;
    let mut extracted: Vec<[String; 2]> = Vec::new();
    for dir in get_dir_files_tree(extract_from_path) {
		if ends_with_any(&dir, &filetypes) {
			e = extract_sheet_from_file(&dir, pattern);
			if e.len() > 0 {
				extracted.push([dir[extract_from_path.len()..].to_string(), String::new()]);
				extracted.extend(e);
			}
			println!("{}", dir);
        }
    }
    return extracted;
}

pub fn save_sheet(path: &str, iter_2d: Vec<[String; 2]>) {
	try_write_string_file(path, &(
		TRANSLATION_SHEET_HEADER.to_owned() + &iter_2d.iter().map(
			|row| "\n".to_owned() + &row[0] + ",\"" + &row[1].replace("\"", "\"\"") + "\""
		).collect::<Vec<String>>().join(""))
	);
}

pub fn enter_to_file(enter_to_path: &str, csv: &Vec<Vec<String>>) {
    let mut bytes: Vec<u8>;
	match fs::read(enter_to_path) {
		Ok(f) => bytes = f,
		Err(_) => return,
	}
    let mut global_offset: isize = 0;
	for row in csv.iter().skip(1) {
        if row[0].len() > 0 && row.len() >= 3 {
            let offset: isize = row[0].parse::<isize>().unwrap();
			bytes.splice(((offset - global_offset) as usize)..((offset - global_offset) as usize + row[1].len()), row[2].as_bytes().iter().cloned());
			global_offset += row[1].len() as isize - row[2].len() as isize;
        }
	}
	try_write_byte_file(enter_to_path, &bytes);
}

pub fn enter_to_folder(enter_to_path: &str, csv: &Vec<Vec<String>>) {
	let mut dir: &str = &csv[1][0];
	let mut bytes: Vec<u8> = fs::read(enter_to_path.to_owned() + dir).unwrap_or(Vec::new());
    let mut global_offset: isize = 0;
	println!("{}", enter_to_path.to_owned() + dir);
	for row in csv.iter().skip(2) {
		if !row[0].is_empty() && !bytes.is_empty() {
			if row[0].chars().next().unwrap() == '\\' {
				try_write_byte_file(&(enter_to_path.to_owned() + dir), &bytes);
				dir = &row[0];
				bytes = fs::read(enter_to_path.to_owned() + dir).unwrap_or(Vec::new());
				global_offset = 0;
				println!("{}", enter_to_path.to_owned() + dir);
			} else if row.len() >= 3 {
				let offset: isize = row[0].parse::<isize>().unwrap();
				bytes.splice(((offset - global_offset) as usize)..((offset - global_offset) as usize + row[1].len()), row[2].as_bytes().iter().cloned());
				global_offset += row[1].len() as isize - row[2].len() as isize;
			}
        }
	}
	try_write_byte_file(&(enter_to_path.to_owned() + dir), &bytes);
}

// ======================= String - textVector

pub fn text_vector_from_string(text: &str, page_com: &str, line_com: &str, coms_pattern: &str) -> Vec<Vec<Vec<String>>> {
	text.split(page_com).map(
		|p| p.split(line_com).map(
			|l|
			split_keep(l, coms_pattern)
		).collect()
	).collect()
}

pub fn string_from_text_vector(text_list: &Vec<Vec<Vec<String>>>, page_com: &str, line_com: &str) -> String {
	text_list.iter().map(
		|p| p.iter().map(
			|l|
			l.join("")
		).collect::<Vec<String>>().join(line_com)
	).collect::<Vec<String>>().join(page_com)
}
