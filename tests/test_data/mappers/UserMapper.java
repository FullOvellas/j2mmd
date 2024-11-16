package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper(uses = {CustomerMapper.class})
public interface UserMapper {
    UserDto toDto(User user);
}

