use regex::Regex;

const MSG1: &str ="\n\n单词：concurrence\n英式音标：/kənˈkʌrəns/\n\n词根词缀：con-（共同）+ -cur-（跑）+ -ence（名词后缀）\n\n中文释义：1. 同时发生；2. 一致；3. 同意；4. 共同参与\n\n常用搭配：in concurrence with（与...同时发生/一致）；concurrence of opinion（意见一致）；concurrence of circumstances（情况相同）\n\n近义词：agreement, harmony, consensus, accord\n\n例句：The concurrence of events led to a successful outcome.（事件的同时发生导致了成功的结果。）";
const MSG2: &str = "单词：rival\n\n英式音标：/ˈraɪvəl/\n\n词根词缀：无\n\n中文释义：竞争对手，敌手\n\n常用搭配：business rival（商业竞争对手），political rival（政治对手），rival company（竞争公司）\n\n近义词：competitor, opponent, adversary\n\n例句：\n\n1. They have been rivals since they were in high school.\n\n自从高中时期以来，他们就是竞争对手。\n\n2. The two companies are fierce rivals in the smartphone market.\n\n这两家公司在智能手机市场上是激烈的竞争对手。";
const MSG3: &str = "单词：apple\n\n英式音标：/ˈæpl/\n\n词根词缀：无\n中文释义：苹果\n常用搭配：an apple a day (每天一苹果)\n近义词：无\n例句：I like to eat apples for breakfast. (我喜欢在早餐时吃苹果。)";

fn capture_message(msg: &str) {
    // capture 单词、英式音标、中文释义、常用搭配、近义词、例句
    let re = Regex::new(
                r#"单词：(?P<word>.+)(\n)+英式音标：(?P<soundmark>.+)(\n)+词根词缀：(?P<roots>.+)(\n)+中文释义：(?P<paraphrase>.+)(\n)+常用搭配：(?P<collocations>.+)(\n)+近义词：(?P<synonyms>.+)(\n)+例句：(?P<examples>(.|\s)+)"#,
    )
    .unwrap();
    let caps = re.captures(msg).unwrap();
    println!("word: {}", caps.name("word").unwrap().as_str());
    println!("soundmark: {}", caps.name("soundmark").unwrap().as_str());
    println!("roots: {}", caps.name("roots").unwrap().as_str());
    println!("paraphrase: {}", caps.name("paraphrase").unwrap().as_str());
    println!(
        "collocations: {}",
        caps.name("collocations").unwrap().as_str()
    );
    println!("synonyms: {}", caps.name("synonyms").unwrap().as_str());
    println!("examples: {}", caps.name("examples").unwrap().as_str());
}

fn capture_msg() {
    const MSG: &str = "x：123\n\ny：abc\n\nccc\n\n";
    let re = Regex::new(r#"x：(?P<word>.+)\n\ny：(?P<number>(.|\s)+)"#).unwrap();
    let caps = re.captures(MSG).unwrap();
    println!("word: {}", caps.name("word").unwrap().as_str());
    println!("number: {}", caps.name("number").unwrap().as_str());
    assert_eq!(caps.name("word").unwrap().as_str(), "123");
    assert_eq!(caps.name("number").unwrap().as_str(), "abc\n\nccc\n\n");
}

fn main() {
    capture_msg();
    capture_message(MSG1);
    capture_message(MSG2);
    capture_message(MSG3);
}
