package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper
public interface ProductMapper {
    String mapToProductName(Product product);
}

