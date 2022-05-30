use crate::protos::SessionApi;
use futures::StreamExt;
use grpc::{
    GrpcStatus, Metadata, ServerRequestSingle, ServerResponseSink, ServerResponseUnarySink,
};
use mizer_api::handlers::SessionHandler;
use mizer_api::models::*;
use mizer_api::RuntimeApi;

impl<R: RuntimeApi + 'static> SessionApi for SessionHandler<R> {
    fn get_session(
        &self,
        req: ServerRequestSingle<SessionRequest>,
        mut resp: ServerResponseSink<Session>,
    ) -> grpc::Result<()> {
        match self.watch_session() {
            Ok(mut stream) => {
                req.loop_handle().spawn(async move {
                    while let Some(m) = stream.next().await {
                        resp.send_data(m)?;
                    }
                    resp.send_trailers(Metadata::new())
                });
                Ok(())
            }
            Err(e) => {
                log::error!("Monitoring of session failed {:?}", e);
                resp.send_grpc_error(
                    GrpcStatus::Internal,
                    format!("Monitoring of session failed {:?}", e),
                )
            }
        }
    }

    fn join_session(
        &self,
        _: ServerRequestSingle<ClientAnnouncement>,
        _: ServerResponseUnarySink<Session>,
    ) -> grpc::Result<()> {
        unimplemented!()
    }

    fn new_project(
        &self,
        _: ServerRequestSingle<ProjectRequest>,
        resp: ServerResponseUnarySink<ProjectResponse>,
    ) -> grpc::Result<()> {
        self.new_project().unwrap();

        resp.finish(Default::default())
    }

    fn load_project(
        &self,
        req: ServerRequestSingle<LoadProjectRequest>,
        resp: ServerResponseUnarySink<ProjectResponse>,
    ) -> grpc::Result<()> {
        self.load_project(req.message.path).unwrap();

        resp.finish(Default::default())
    }

    fn save_project(
        &self,
        _: ServerRequestSingle<ProjectRequest>,
        resp: ServerResponseUnarySink<ProjectResponse>,
    ) -> grpc::Result<()> {
        self.save_project().unwrap();

        resp.finish(Default::default())
    }

    fn save_project_as(
        &self,
        req: ServerRequestSingle<super::session::SaveProjectAsRequest>,
        resp: ServerResponseUnarySink<ProjectResponse>,
    ) -> grpc::Result<()> {
        self.save_project_as(req.message.path).unwrap();

        resp.finish(Default::default())
    }

    fn load_history(
        &self,
        req: ServerRequestSingle<LoadHistoryRequest>,
        resp: ServerResponseSink<History>,
    ) -> grpc::Result<()> {
        todo!()
    }
}
