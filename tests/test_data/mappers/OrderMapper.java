package com.example.mappers;

import org.mapstruct.Mapper;

@Mapper(uses = {
                ProductMapper.class,
                AddressMapper.class
})
public interface OrderMapper {
    OrderDto toDto(Order order);
}

