use super::*;

impl<T: AsTypename + AsVariable> SerializerMemberData for T {
    fn def_member_data(&self, writer: &mut Writer) {
        writer.def_member_var(&self.as_typename(), &self.as_variable(Some("_"), Some("_data")), None);
    }

    fn def_member_set(&self, writer: &mut Writer) {
        writer.def_member_var(&self.as_typename(), &self.as_variable(Some("_"), Some("_set")), None);
    }

    fn member_set(&self) -> String {
        self.as_variable(Some("_"), Some("_set"))
    }

    fn member_data(&self) -> String {
        self.as_variable(Some("_"), Some("_data"))
    }
}
