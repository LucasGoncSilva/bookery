CREATE TABLE IF NOT EXISTS tbl_rentals (
  id UUID PRIMARY KEY NOT NULL,
  costumer_uuid UUID REFERENCES tbl_costumers(id),
  book_uuid UUID REFERENCES tbl_books(id),
  borrowed_at DATE NOT NULL,
  due_date DATE NOT NULL DEFAULT (CURRENT_DATE + INTERVAL '30 days'),
  returned_at DATE DEFAULT NULL
)
