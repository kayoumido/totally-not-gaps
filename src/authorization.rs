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
