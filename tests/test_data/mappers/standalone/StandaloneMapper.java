package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper
public interface StandaloneMapper {
    String mapToUpperCase(String input);
}

