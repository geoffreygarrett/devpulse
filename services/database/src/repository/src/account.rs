use async_trait::async_trait;

pub struct GenericAccountRepository<D>
where
    D: DataAccess<Account>,
{
    data_accessor: D,
}

impl<D> GenericAccountRepository<D>
where
    D: DataAccess<Account>,
{
    pub fn new(data_accessor: D) -> Self {
        Self { data_accessor }
    }
}

#[async_trait]
impl<D> AccountRepository for GenericAccountRepository<D>
where
    D: DataAccess<Account> + Sync + Send,
{
    type Error = D::Error;

    async fn find_by_id(&self, id: u64) -> Result<Option<Account>, Self::Error> {
        self.data_accessor.read(id).await
    }

    async fn save(&self, account: &Account) -> Result<(), Self::Error> {
        self.data_accessor.create(account).await
    }

    async fn update(&self, account: &Account) -> Result<(), Self::Error> {
        self.data_accessor.update(account).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), Self::Error> {
        self.data_accessor.delete(id).await
    }
}
