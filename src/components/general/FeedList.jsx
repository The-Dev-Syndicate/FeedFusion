import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';

export default function FeedList() {
    const [feeds, setFeeds] = useState([]);

    useEffect(() => {
        invoke('load_feeds')
            .then((items) => {
                const uniqueFeeds = removeDuplicates(feeds.concat(items));
                setFeeds(uniqueFeeds);
            })
            .catch(console.error);
    }, []);

    // Function to remove duplicates from an array of feeds
    const removeDuplicates = (arr) => {
        const uniqueMap = new Map();
        arr.forEach((feed) => {
            uniqueMap.set(feed.url, feed);
        });
        return Array.from(uniqueMap.values());
    };

    return (
        <ul>
            {feeds.map((feed, index) => (
                <li key={index} className='list-item'>
                    {feed.alias ?? feed.url}
                </li>
            ))}
        </ul>
    );
}