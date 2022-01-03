use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;
use crate::Console;

/// Utility function to read an array of types from a serialised game object.
pub(crate) fn construct_array<T>(
    bin: &Bin,
    initial_object_offset: usize,
    console: Console,
    array_offset: usize,
    array_count_offset: usize,
) -> Result<Vec<T>, Error>
where
    T: SerialisedShrekSuperSlamGameObject,
{
    let list_offset = console.read_u32(
        &bin.raw[initial_object_offset + array_offset..initial_object_offset + array_offset + 4],
    )? as usize;
    let list_count = console.read_u32(
        &bin.raw[initial_object_offset + array_count_offset
            ..initial_object_offset + array_count_offset + 4],
    )? as usize;
    let constructed_list: Result<Vec<T>, Error> = (0..list_count)
        .map(|i| {
            let list_entry_offset =
                (Bin::header_length() + list_offset + (i * 4)) as usize;
            console.read_u32(&bin.raw[list_entry_offset..list_entry_offset + 4])
        })
        .map(|offset| bin.get_object_from_offset::<T>(offset?))
        .collect();

    constructed_list
}

/// Utility function to construct an optional type from a serialised game object.
pub(crate) fn construct_optional_type<T>(
    bin: &Bin,
    initial_object_offset: usize,
    console: Console,
    object_pointer_offset: usize,
) -> Result<Option<T>, Error>
where
    T: SerialisedShrekSuperSlamGameObject,
{
    let object_offset = console.read_u32(
        &bin.raw[initial_object_offset + object_pointer_offset
            ..initial_object_offset + object_pointer_offset + 4],
    )?;
    let object = if object_offset != 0 {
        Some(bin.get_object_from_offset::<T>(object_offset)?)
    } else {
        None
    };

    Ok(object)
}
