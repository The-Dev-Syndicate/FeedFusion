// src/components/general/Navbar.jsx
import React from 'react';
import { NavLink } from 'react-router-dom';

export default function() {
    return (
        <nav className="navbar">
            <NavLink to="/" exact activeClassName="active">Home</NavLink>
            <NavLink to="/about" activeClassName="active">About</NavLink>
            <NavLink to="/asdasd" activeClassName="active">404</NavLink>
        </nav>
    );
};

