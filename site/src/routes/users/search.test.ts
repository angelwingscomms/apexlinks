import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import userService from '$lib/services/userService';
import UserSearchForm from '$lib/components/UserSearchForm.svelte';
import UsersPage from './+page.svelte';

// Mock the user service
vi.mock('$lib/services/userService', () => ({
  default: {
    searchUsers: vi.fn()
  }
}));

// Mock the isAuthenticated store
vi.mock('$lib/stores/userStore', () => ({
  isAuthenticated: { subscribe: vi.fn().mockImplementation((fn) => {
    fn(true);
    return () => {};
  })},
  currentUser: { subscribe: vi.fn().mockImplementation((fn) => {
    fn({ id: 'testuser' });
    return () => {};
  })}
}));

describe('User Search Form', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should emit search event with correct parameters when searching with text only', async () => {
    const { component } = render(UserSearchForm);
    
    // Set up event listener
    const mockFn = vi.fn();
    component.$on('search', mockFn);
    
    // Fill in search query
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Check that the event was emitted with the correct parameters
    expect(mockFn).toHaveBeenCalled();
    const emittedEvent = mockFn.mock.calls[0][0].detail;
    expect(emittedEvent).toEqual({
      query: 'test query',
      s: 'u',
      limit: 50
    });
  });
  
  it('should include age filter when min and max age are set', async () => {
    const { component } = render(UserSearchForm);
    
    // Set up event listener
    const mockFn = vi.fn();
    component.$on('search', mockFn);
    
    // Fill in search parameters
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Set min and max age
    const minAgeInput = screen.getByPlaceholderText('Min');
    const maxAgeInput = screen.getByPlaceholderText('Max');
    await fireEvent.input(minAgeInput, { target: { value: '25' } });
    await fireEvent.input(maxAgeInput, { target: { value: '40' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Check that the event was emitted with the correct parameters
    expect(mockFn).toHaveBeenCalled();
    const emittedEvent = mockFn.mock.calls[0][0].detail;
    expect(emittedEvent).toEqual({
      query: 'test query',
      s: 'u',
      limit: 50,
      minAge: 25,
      maxAge: 40
    });
  });
  
  it('should include gender filter when gender is selected', async () => {
    const { component } = render(UserSearchForm);
    
    // Set up event listener
    const mockFn = vi.fn();
    component.$on('search', mockFn);
    
    // Fill in search query
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Select gender
    const genderSelect = screen.getByLabelText('Gender');
    await fireEvent.change(genderSelect, { target: { value: 'female' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Check that the event was emitted with the correct parameters
    expect(mockFn).toHaveBeenCalled();
    const emittedEvent = mockFn.mock.calls[0][0].detail;
    expect(emittedEvent).toEqual({
      query: 'test query',
      s: 'u',
      limit: 50,
      gender: 'female'
    });
  });
  
  it('should reset all form fields when reset button is clicked', async () => {
    render(UserSearchForm);
    
    // Fill in search parameters
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Set min and max age
    const minAgeInput = screen.getByPlaceholderText('Min');
    const maxAgeInput = screen.getByPlaceholderText('Max');
    await fireEvent.input(minAgeInput, { target: { value: '25' } });
    await fireEvent.input(maxAgeInput, { target: { value: '40' } });
    
    // Select gender
    const genderSelect = screen.getByLabelText('Gender');
    await fireEvent.change(genderSelect, { target: { value: 'female' } });
    
    // Click reset button
    const resetButton = screen.getByRole('button', { name: 'Reset' });
    await fireEvent.click(resetButton);
    
    // Check that all fields are reset
    expect(searchInput).toHaveValue('');
    expect(minAgeInput).toHaveValue(null);
    expect(maxAgeInput).toHaveValue(null);
    expect(genderSelect).toHaveValue('');
  });
});

describe('Users Page Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Mock successful response
    (userService.searchUsers as any).mockResolvedValue([
      { id: 'user1', name: 'Test User 1', description: 'Description 1' },
      { id: 'user2', name: 'Test User 2', description: 'Description 2' }
    ]);
  });
  
  it('should display loading state and then results when search is performed', async () => {
    render(UsersPage);
    
    // Fill in search query in the form within the page
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Check loading state is displayed
    expect(screen.getByText('Searching for users...')).toBeInTheDocument();
    
    // Wait for results to be displayed
    await waitFor(() => {
      expect(userService.searchUsers).toHaveBeenCalledWith({
        query: 'test query',
        s: 'u',
        limit: 50
      });
      
      // Check that user cards are displayed
      expect(screen.getByText('Test User 1')).toBeInTheDocument();
      expect(screen.getByText('Test User 2')).toBeInTheDocument();
    });
  });
  
  it('should display error message when search fails', async () => {
    // Mock a failed search
    (userService.searchUsers as any).mockRejectedValue(new Error('API error'));
    
    render(UsersPage);
    
    // Fill in search query in the form within the page
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Wait for error message to be displayed
    await waitFor(() => {
      expect(screen.getByText('Failed to search users. Please try again.')).toBeInTheDocument();
    });
  });
  
  it('should display "No Users Found" message when search returns empty results', async () => {
    // Mock an empty response
    (userService.searchUsers as any).mockResolvedValue([]);
    
    render(UsersPage);
    
    // Fill in search query in the form within the page
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Wait for "No Users Found" message to be displayed
    await waitFor(() => {
      expect(screen.getByText('No Users Found')).toBeInTheDocument();
      expect(screen.getByText('Try adjusting your search criteria.')).toBeInTheDocument();
    });
  });
  
  it('should handle age filters correctly in API calls', async () => {
    render(UsersPage);
    
    // Fill in search query
    const searchInput = screen.getByPlaceholderText('Search by name, username, or description');
    await fireEvent.input(searchInput, { target: { value: 'test query' } });
    
    // Set min and max age
    const minAgeInput = screen.getByPlaceholderText('Min');
    const maxAgeInput = screen.getByPlaceholderText('Max');
    await fireEvent.input(minAgeInput, { target: { value: '25' } });
    await fireEvent.input(maxAgeInput, { target: { value: '40' } });
    
    // Submit the form
    const searchButton = screen.getByRole('button', { name: 'Search' });
    await fireEvent.click(searchButton);
    
    // Wait for the API call to be made
    await waitFor(() => {
      // First check that searchUsers was called
      expect(userService.searchUsers).toHaveBeenCalled();
      
      // Then check that the first argument had the expected properties
      const apiCallArg = (userService.searchUsers as any).mock.calls[0][0];
      expect(apiCallArg).toHaveProperty('minAge', 25);
      expect(apiCallArg).toHaveProperty('maxAge', 40);
    });
  });
}); 