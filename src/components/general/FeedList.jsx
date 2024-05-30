import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';

export default function FeedList() {
    const [feeds, setFeeds] = useState([]);

    useEffect(() => {
        fetchFeeds();
    }, []);

    const fetchFeeds = () => {
        invoke('load_feeds').then((response) => {
            setFeeds(response);
        }).catch((error) => {
            console.error('Error loading feeds:', error);
        });
    };

    return (
        <ul>
            {feeds.map((feed, index) => (
                <li className="list-item" key={index}>
                    {feed.alias ? feed.alias : feed.url}
                </li>
            ))}
        </ul>
    );
}