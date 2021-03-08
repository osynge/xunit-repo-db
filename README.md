# Diesel database for [xunit-repo](https://github.com/osynge/xunit-repo).

## To migrate db:

```
rm repo.db src/schema.rs
diesel migration run
```

## TODO

* Using cargo features add support for pgsql.