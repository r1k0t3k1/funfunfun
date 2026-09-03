use application::domain::model::role_model::Role;
use application::outbound::agent_repository::AgentRepository;
use sqlx::PgPool;

use application::domain::model::operator_model::OperatorModel;
use application::outbound::error::RepositoryError;
use application::outbound::operator_repository::OperatorRepository;

use crate::entity::operator_entity::OperatorEntity;
use crate::entity::role_entity::RoleEntity;

#[derive(Debug, Clone)]
pub struct AgentRepositoryImpl {
    connection: PgPool,
}

impl AgentRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

impl AgentRepository for AgentRepositoryImpl {
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn find_by_id<'life0,'async_trait>(&'life0 self,agent_id: String) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<AgentModel> ,RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn list<'life0,'async_trait>(&'life0 self) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<AgentModel> ,RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn insert<'life0,'async_trait>(&'life0 self,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AgentModel,RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn save<'life0,'async_trait>(&'life0 self,agent: AgentModel) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AgentModel,RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn start_by_id<'life0,'async_trait>(&'life0 self,agent_id: AgentId) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<(),RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn stop_by_id<'life0,'async_trait>(&'life0 self,agent_id: AgentId) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<(),RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn delete_by_id<'life0,'async_trait>(&'life0 self,agent_id: AgentId) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<(),RepositoryError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }
}
