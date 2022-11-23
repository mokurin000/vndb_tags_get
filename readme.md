# vndb_tag_get

parse vndb tag list and sort by num of VN it belongs, then output as a spreadsheet in markdown

[VNDB tag list](https://dl.vndb.org/dump/vndb-tags-latest.json.gz)

## Usage

```bash
curl -L https://dl.vndb.org/dump/vndb-tags-latest.json.gz | gzip -d | vndb_tags_get > vndb_tags.md
```
