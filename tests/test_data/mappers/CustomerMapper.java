package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper(uses = AddressMapper.class)
public interface CustomerMapper {
    CustomerDto toDto(Customer customer);
}

