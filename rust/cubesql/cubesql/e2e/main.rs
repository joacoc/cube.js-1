use std::fmt::Debug;

use tests::{basic::AsyncTestSuite, mysql::MySqlIntegrationTestSuite};

pub mod tests;

#[derive(Debug)]
struct TestsRunner {
    pub suites: Vec<Box<dyn AsyncTestSuite>>,
}

impl TestsRunner {
    pub fn new() -> Self {
        Self { suites: Vec::new() }
    }

    pub fn register_suite(&mut self, suite: Box<dyn AsyncTestSuite>) {
        self.suites.push(suite);
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut runner = TestsRunner::new();
        runner.register_suite(MySqlIntegrationTestSuite::before_all().await);

        for suites in runner.suites.iter_mut() {
            suites.run().await.unwrap();
        }
    });
}