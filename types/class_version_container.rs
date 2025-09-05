#[derive(Clone, Debug)]
pub struct ClassVersionContainer {
    pub structure_version: u32,
    pub class_versions: Map<String, u16>,
}

impl ClassVersionContainer {
    pub fn new() -> Self {
        ClassVersionContainer {
            structure_version: 0,
            class_versions: Map::new(),
        }
    }

    pub fn format_to_string(&self, indentation_level: usize) -> String {
        let indent_values = "\t".repeat(indentation_level + 1);
        let indent_end = "\t".repeat(indentation_level);

        let mut s = String::new();
        s.push_str("ClassVersionContainer{\n");
        s.push_str(&format!("{}StructureVersion: {},\n", indent_values, self.structure_version));
        s.push_str(&format!("{}ClassVersions: {}\n", indent_values, self.class_versions));
        s.push_str(&format!("{}}}", indent_end));
        s
    }
}

impl RVType for ClassVersionContainer {
    fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
        self.class_versions.write_to(writable)
    }

    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
        self.class_versions.extract_from(readable)
    }

    fn copy(&self) -> Box<dyn RVType> {
        Box::new(self.clone())
    }

    fn equals(&self, other: &dyn RVType) -> bool {
        other.as_any().downcast_ref::<ClassVersionContainer>()
            .map_or(false, |o| self.class_versions == o.class_versions)
    }

    fn copy_ref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn deref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for ClassVersionContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_to_string(0))
    }
}
