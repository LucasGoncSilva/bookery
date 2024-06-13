CREATE TABLE IF NOT EXISTS tbl_books (
  id UUID PRIMARY KEY NOT NULL,
  name VARCHAR(64) NOT NULL,
  author_uuid UUID REFERENCES tbl_authors(id),
  editor VARCHAR(64) NOT NULL,
  release DATE NOT NULL
)
