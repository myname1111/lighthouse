use std::mem::size_of;

use crate::graphics::{buffer::*, vertex::VertexArray};

use super::world::{GameObjectTrait, World};
use nalgebra_glm::*;
use ogl33::*;

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

/// Creates a new game object
pub trait Object<GameObject: GameObjectTrait + Sized>: PosRot {
    /// update the object
    fn update(world: &mut World<GameObject>, index: usize)
    where
        Self: Sized;
}

/// An object trait that if implemented,
/// your object can be controlled by your keyboard
///
/// # Examples
/// this example explains how to implement your trait for your object
/// ```
/// impl ControllableKey for MyObject {
///     fn on_key() -> fn(world: &mut World, index: usize);
///         |world, index| {
///             // get all keys that are pressed
///             for key in keys {
///                 // match keys
///                 match key {
///                     Keycode::A => println!("Key a is pressed"),
///                 }
///             }
///         }
///     }
/// }
/// ```
pub trait ControllableKey<GameObject: GameObjectTrait + Sized> {
    /// Do things with device on update
    fn on_key(world: &mut World<GameObject>);
}

/// An object trait that if implemented,
/// your object can be controlled by your mouse
///
/// # Examples
/// this example explains how to implement your trait for your object
/// ```
/// // in on_mouse
/// for pressed in mouse.get_pressed_cooldown(Duration::from_millis(100)) {
///     match pressed {
///         MousePressed::LeftMouse => println!("Left mouse pressed"),
///         MousePressed::RightMouse => println!("Right mouse pressed"),
///         _ => (),
///     }
/// }
/// ```
pub trait ControllableMouse<GameObject: GameObjectTrait + Sized> {
    /// Do things with device on update
    fn on_mouse(world: &mut World<GameObject>);
}

/// A vertex for your object
pub trait VertexTrait: Copy {
    /// How many elements are in a vertex
    /// e.g. Vertex { foo: [1, 2], bar: [3, 4]} = 4
    const SIZE: usize;

    /// return the vertex as a vector of len SIZE
    fn as_list(&self) -> Vec<f32>;
}

/// Mesh for your object
pub struct Mesh<Vertex: VertexTrait> {
    /// The vertices of your object
    pub vertices: Vec<Vertex>,
    /// This is the size of the vertex attributes
    pub vert_attr: Vec<usize>,
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
    pub indicies: Vec<[usize; 3]>,
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
}

impl<Vertex: VertexTrait> Mesh<Vertex> {
    /// Creates a new Mesh
    pub fn new(
        vert: Vec<Vertex>,
        vert_attr: Vec<usize>,
        index: Vec<[usize; 3]>,
    ) -> Result<Mesh<Vertex>, String> {
        if vert[0].as_list().len() != (&vert_attr).into_iter().sum() {
            return Err("The sum of the vertex attributes must be equal to the number of element in the vertex".into());
        }

        let out = Mesh {
            vertices: vert,
            vert_attr,
            indicies: index,
            vao: VertexArray::new().expect("Couldn't make a VAO"),
            vbo: Buffer::new().expect("Couldn't make a VBO"),
            ebo: Buffer::new().expect("Couldn't make EBO"),
        };

        out.vao.bind();
        out.vbo.bind(BufferType::Array);
        out.ebo.bind(BufferType::ElementArray);

        Ok(out)
    }

    /// Updates the mesh
    pub fn update_mesh(&self) {
        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(
                &self
                    .vertices
                    .clone()
                    .into_iter()
                    .flat_map(|vertex| vertex.as_list())
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

    /// Sets the vertex atrributes that will later on be passed into layout
    pub fn set_vert_attr(&self) {
        for (i, attr) in (&self.vert_attr).into_iter().enumerate() {
            let pointer = size_of::<f32>() * self.vert_attr[0..i].into_iter().sum::<usize>();

            unsafe {
                glVertexAttribPointer(
                    i.try_into().unwrap(),
                    (*attr).try_into().unwrap(),
                    GL_FLOAT,
                    GL_FALSE,
                    size_of::<Vertex>().try_into().unwrap(),
                    pointer as *const _,
                )
            }
        }
    }
}

/// Implement this trait if your object has a mesh
pub trait MeshTrait<GameObject, Vertex>: Object<GameObject>
where
    GameObject: GameObjectTrait + Sized,
    Vertex: VertexTrait,
{
    /// gets the mesh
    fn get_mesh(&self) -> Mesh<Vertex>;
}
