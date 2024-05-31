import React from 'react';
import { NavLink } from 'react-router-dom';

export default function Navbar() {
    return (
        <nav className="navbar">
            <NavLink
                to="/"
                end
                className={({ isActive }) => isActive ? 'active' : ''}
            >
                Home
            </NavLink>
            <NavLink
                to="/about"
                className={({ isActive }) => isActive ? 'active' : ''}
            >
                About
            </NavLink>
            <NavLink
                to="/asdasd"
                className={({ isActive }) => isActive ? 'active' : ''}
            >
                404
            </NavLink>
        </nav>
    );
}
