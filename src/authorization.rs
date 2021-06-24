use casbin::prelude::*;

const CONFIG: &str = "accessControl/access_control.conf";
const POLICY: &str = "accessControl/access_control.csv";

///Centralized access control mechanism
pub async fn auth(subject: &str, ressource: &str, action: &str) -> bool {
    let e = Enforcer::new(CONFIG, POLICY)
        .await
        .expect("cannot read model or policy");
    if let Ok(authorized) = e.enforce((subject, ressource, action)) {
        authorized
    } else {
        panic!("Casbin model does not map request");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::executor::block_on;
    use rstest::rstest;

    #[rstest(
        subject,
        ressource,
        action,
        expected,
        case("teacher", "grades", "read", true),
        case("student", "grades", "read", true),
        case("teacher", "grades", "write", true),
        case("student", "grades", "write", false),
        case("student", "unknown", "write", false),
        case("teacher", "grades", "execute", false),
        case("teacher", "what", "doing", false),
        ::trace
    )]
    fn tet_authorization(subject: &str, ressource: &str, action: &str, expected: bool) {
        assert_eq!(block_on(auth(subject, ressource, action)), expected);
    }
}
