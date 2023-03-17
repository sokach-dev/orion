# orion

## intro

`orion` is a tools, that used to learn vocabulary depend on ChatGPT generate story and prectice writting.

## vocabulary service
```bash
RUST_LOG=info ./target/debug/orion rpc -c rpc.toml
2023-03-16 16:52:43.610  INFO service: service/src/lib.rs:44: Listening on 0.0.0.0:5015
```

## 查询

```bash
 grpcurl -plaintext -import-path abi/protos/ -proto orion.proto -d '{"query":{ "word": "apple"}}' 127.0.0.1:5015 orion.VocabularyService/QueryVocabulary
{
  "vocabulary": [
    {
      "id": "314",
      "word": "apple",
      "soundmark": "/ˈæpl/",
      "roots": "无",
      "paraphrase": "苹果",
      "collocations": "an apple a day (一天一苹果)",
      "synonyms": "无",
      "examples": "I ate an apple for breakfast. (我早餐吃了一个苹果。)",
      "createdAt": "2023-03-15T01:07:39.976009Z",
      "updatedAt": "2023-03-15T01:07:39.976009Z"
    }
  ]
}

```

## 进度

### abi
- [x] 提供配置解析能力
- [x] 提供proto文件解析能力

### service
- [x] 对orion.vocabulary表的增删查改
- [x] 提供orion.vocabulary表的增删查改的rpc接口

### openai
- [x] 对访问openai进行封装

### vocabulary
- [x] 通过openai添加新单词

### tui
- [ ] 基本的查询能力
