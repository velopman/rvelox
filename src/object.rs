pub trait ObjTrait {
    fn size(&self) -> usize;

}

#[derive(Hash)]
pub struct ObjRef<T: ObjTrait> {
    index: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ObjTrait> Clone for ObjRef<T> {
    fn clone(&self) -> ObjRef<T> {
        *self
    }
}

impl<T: ObjTrait> Copy for ObjRef<T> {}

impl<T: ObjTrait> Eq for ObjRef<T> {}

// impl hash::Hash for ObjRef<String> {
//     fn hash<H: hash::Hasher>(&self, state: &mut H) {
//         return self.index.hash(state);
//     }
// }

impl ObjTrait for String {
    fn size(&self) -> usize {
        return std::mem::size_of::<String>() + self.as_bytes().len();
    }
}

impl<T: ObjTrait> PartialEq for ObjRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub struct ObjAllocator {
    objects: Vec<ObjHeader>, // TODO: Make optional with GC
    strings: std::collections::HashMap<String, ObjRef<String>>,
}

impl ObjAllocator {
    pub fn new() -> ObjAllocator {
        ObjAllocator {
            objects: Vec::new(),
            strings: std::collections::HashMap::new(),
        }
    }

    pub fn alloc<T: ObjTrait + 'static>(&mut self, obj: T) -> ObjRef<T> {
        let size: usize = obj.size() + std::mem::size_of::<ObjHeader>();

        let entry: ObjHeader = ObjHeader {
            size,
            obj: Box::new(obj),
        };

        self.objects.push(entry);
        let index: usize = self.objects.len();

        return ObjRef {
            index,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn intern(&mut self, name: String) -> ObjRef<String> {
        match self.strings.get(&name) {
            Some(&value) => value,
            None => {
                let reference: ObjRef<String> = self.alloc(name);
                self.strings.insert(name, reference);

                reference
            }
        }
    }

    pub fn deref<T: ObjTrait>(&self, reference: ObjRef<T>) -> &T {
        self.objects[reference.index]
            .as_ref()
            .unwrap()
            .obj
            .as_any()
            .downcast_ref()
            .unwrap_or_else(|| None /* TODO: Panic */);
    }
}

struct ObjHeader {
    size: usize,
    obj: Box<dyn ObjTrait>,
}
