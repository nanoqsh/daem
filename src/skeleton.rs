use {
    serde::Serialize,
    std::{collections::HashMap, fmt},
};

#[derive(Default, Clone, Serialize)]
#[serde(into = "Vec<Bone>")]
pub(crate) struct Skeleton {
    bones: Vec<Bone>,
    names: HashMap<String, u16>,
}

impl Skeleton {
    pub fn push(&mut self, name: String, bone: Bone) -> Result<(), ToManyBones> {
        let id = self.bones.len().try_into().map_err(|_| ToManyBones)?;
        self.bones.push(bone);
        self.names.insert(name, id);

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.bones.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<u16> {
        self.names.get(name).copied()
    }
}

impl From<Skeleton> for Vec<Bone> {
    fn from(Skeleton { bones, .. }: Skeleton) -> Self {
        bones
    }
}

pub(crate) struct ToManyBones;

impl fmt::Display for ToManyBones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "to many bones")
    }
}

#[derive(Clone, Serialize)]
pub(crate) struct Bone {
    pub name: String,
    pub pos: [f32; 3],
    pub rot: [f32; 4],
    pub parent: Option<u16>,
}
