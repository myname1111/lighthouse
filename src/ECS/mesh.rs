use std::mem::size_of;

use crate::graphics::{buffer::*, vertex::VertexArray, *};
use ogl33::*;

use super::*;
use nalgebra_glm::*;

/// Sets and gets the position and rotaion of the object
pub trait PosRot {
    /// Get the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// let object_pos = object.get_pos();
    ///
    /// // Print position
    /// println!("{:?}", object_pos);
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn print_pos(&self) {
    ///         println!("{}", self.get_pos())
    ///     }
    /// }
    /// ```
    fn get_pos(&self) -> &Vec3;
    /// Get the rotation of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// let object_rot = object.get_rot();
    ///
    /// // Print position
    /// println!("{:?}", object_rot);
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn print_pos(&self) {
    ///         println!("{}", self.get_rot())
    ///     }
    /// }
    /// ```
    fn get_rot(&self) -> &Vec4;

    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_rot() = vec3(0.0, 0.0, 0.0);
    ///
    /// // Print position
    /// println!("{:?}", object.get_pos());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_pos(&mut self) {
    ///         self.set_rot() += 0.1
    ///     }
    /// }
    /// ```
    fn set_pos(&mut self) -> &mut Vec3;
    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_rot() = vec3(0.0, 0.0, 0.0);
    ///
    /// // Print position
    /// println!("{:?}", object.get_rot());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_rot(&mut self) {
    ///         self.set_rot() += 0.1
    ///     }
    /// }
    /// ```
    fn set_rot(&mut self) -> &mut Vec4;
}

#[macro_export]
/// Automaticly implement [PosRot] for you
///
/// # Example
///
/// basic usage
/// ```
/// struct MyObject {
///     pos: Vec3,
///     rot: Vec4 // these two are needed to make impl_posrot work
/// }
///
/// impl_posrot(MyObject) // this will implement posrot for you
/// ```
///
/// todo: A derive would be better
macro_rules! impl_posrot {
    ($object: ident) => {
        impl PosRot for $object {
            fn get_pos(&self) -> &Vec3 {
                &self.pos
            }

            fn get_rot(&self) -> &Vec4 {
                &self.rot
            }

            fn set_pos(&mut self) -> &mut Vec3 {
                &mut self.pos
            }

            fn set_rot(&mut self) -> &mut Vec4 {
                &mut self.rot
            }
        }
    };
}

/// A vertex for your object
pub trait VertexTrait: Copy {
    /// How many elements are in a vertex
    /// e.g. Vertex { foo: [1, 2], bar: [3, 4]} = 4
    const SIZE: u32;

    /// return the vertex as a vector of len SIZE
    fn as_list(&self) -> Vec<f32>;

    /// Returns the vertex after it has been transformed
    /// rotated and translated
    fn get_vertex(&self, pos: Vec3, rot: Vec4) -> Self;
}

/// Mesh for your object
#[derive(Component)]
pub struct Mesh<Vertex: VertexTrait + 'static + Sync + Send> {
    /// The vertices of your object
    pub vertices: Vec<Vertex>,
    /// This is the size of the vertex attributes
    pub vert_attr: Vec<u32>,
    /// The indicies for vertices
    /// # Example
    /// ```
    /// Mesh<Vertex> {
    ///     vertices: vec![
    ///         Vertex([1, 2, 3]),
    ///         Vertex([3, 2, 1]),
    ///         Vertex([5, 9, 3]),
    ///         Vertex([2, 1, 3])
    ///         // -snip-
    ///     ]
    ///     indicies: vec![[1, 2, 3], [2, 1, 2], [2, 1, 1]]
    /// }
    /// ```
    pub indicies: Vec<[u32; 3]>,
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
}

impl<Vertex: VertexTrait + 'static + Sync + Send> Mesh<Vertex> {
    /// Creates a new Mesh
    pub fn new(
        vert: Vec<Vertex>,
        vert_attr: Vec<u32>,
        index: Vec<[u32; 3]>,
    ) -> Result<Mesh<Vertex>, String> {
        if vert[0].as_list().len() != (&vert_attr).iter().sum::<u32>().try_into().unwrap() {
            return Err(format!("The sum of the vertex attributes {} must be equal to the number of element in the vertex {}", (&vert_attr).iter().sum::<u32>(), vert[0].as_list().len()));
        }

        let out = Mesh {
            vertices: vert,
            vert_attr,
            indicies: index,
            vao: VertexArray::new().expect("Couldn't make a VAO"),
            vbo: Buffer::new().expect("Couldn't make a VBO"),
            ebo: Buffer::new().expect("Couldn't make EBO"),
        };

        Ok(out)
    }

    /// Setsup the mesh, is used for macro
    pub fn setup(&self) {
        self.vao.bind();
        self.vbo.bind(BufferType::Array);
        self.ebo.bind(BufferType::ElementArray);

        for (i, attr) in (&self.vert_attr).iter().enumerate() {
            let pointer: u32 = size_of::<f32>().try_into().unwrap();
            let pointer = pointer * self.vert_attr[0..i].iter().sum::<u32>();

            unsafe {
                glVertexAttribPointer(
                    i.try_into().unwrap(),
                    (*attr).try_into().unwrap(),
                    GL_FLOAT,
                    GL_FALSE,
                    size_of::<Vertex>().try_into().unwrap(),
                    pointer as *const _,
                );

                glEnableVertexAttribArray(i.try_into().unwrap())
            }
        }
    }

    /// Updates the mesh
    fn update(&self, pos: Position, rot: Rotation) {
        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(
                &self
                    .vertices
                    .clone()
                    .iter()
                    .flat_map(|vertex| vertex.get_vertex(pos.0, rot.0).as_list())
                    .collect::<Vec<f32>>(),
            ),
            GL_STATIC_DRAW,
        );
        buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(&self.indicies),
            GL_STATIC_DRAW,
        );
    }
}
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Rotation(Vec4);

#[macro_export]
/// implement setup methods systems
/// struct_name: the name of a struct
/// vertex: the vertex
macro_rules! impl_setup_mesh {
    ($struct_name:ident, $vertex:ident) => {
        struct $struct_name;

        impl<'a> System<'a> for $struct_name {
            type SystemData = ReadStorage<'a, Mesh<$vertex>>;

            fn run(&mut self, mesh_vec: Self::SystemData) {
                for mesh in mesh_vec.join() {
                    mesh.setup()
                }
            }
        }
    };
}

#[macro_export]
/// implement update methods systems
/// struct_name: the name of a struct
/// vertex: the vertex
macro_rules! impl_update_mesh {
    ($struct_name:ident, $vertex:ident) => {
        struct $struct_name;

        impl<'a> System<'a> for $struct_name {
            type SystemData = (
                ReadStorage<'a, Position>,
                ReadStorage<'a, Rotation>,
                ReadStorage<'a, Mesh<$vertex>>,
            );

            fn run(&mut self, (pos_vec, rot_vec, mesh_vec): Self::SystemData) {
                for (pos, rot, mesh) in (&pos_vec, &rot_vec, &mesh_vec).join() {
                    mesh.update(pos, rot)
                }
            }
        }
    };
}
