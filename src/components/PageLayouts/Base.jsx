import React from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from "../general/Sidebar";
import Navbar from '../general/Navbar';

export default function Base() {
    return (
        <div className="container">
            <div className="main-content">
                <Sidebar />
                <div className="content">
                    <Outlet />
                </div>
            </div>
            <Navbar />
        </div>
    );
}