use crate::common::errors::Res;
use crate::interactor::book::book_model::Book;

pub struct BookRepo {
    book_repo: super::BookDataStore,
}

impl BookRepo {
    pub fn new(book_repo: super::BookDataStore) -> Self {
        Self { book_repo }
    }

    pub async fn create_book(&self, book: &Book) -> Res<()> {
        let res = self.book_repo.create_book(book).await?;
        Ok(res)
    }

    pub async fn list_book(&self) -> Res<Vec<Book>> {
        // let res = self.book_repo.list_book().await?;
        // Ok(res.0.into_iter().map(Book::from).collect())
        Ok(Vec::new())
    }
}
