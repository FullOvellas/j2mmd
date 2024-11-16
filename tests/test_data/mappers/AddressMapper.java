package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper
public interface AddressMapper {
    String mapToStreet(Address address);
}

