import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { toast } from 'sonner';
import { UserForm } from '../../components/admin';
import { usersApi } from '../../api/users';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { UserResponse, CreateUserRequest } from '../../types/user';

export function EditUserPage() {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();

  const [user, setUser] = useState<UserResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchUser = async () => {
      if (!id) {
        navigate('/admin/users');
        return;
      }

      setIsLoading(true);
      try {
        const userData = await usersApi.get(id);
        setUser(userData);
      } catch (err) {
        setError(mapErrorToMessage(err));
      } finally {
        setIsLoading(false);
      }
    };

    fetchUser();
  }, [id, navigate]);

  const handleSubmit = async (data: CreateUserRequest) => {
    if (!id) return;

    setIsSaving(true);
    setError('');
    try {
      await usersApi.update(id, data);
      toast.success('User updated successfully');
      navigate('/admin/users');
    } catch (err) {
      setError(mapErrorToMessage(err));
    } finally {
      setIsSaving(false);
    }
  };

  const handleCancel = () => {
    navigate('/admin/users');
  };

  if (isLoading) {
    return (
      <div className="container mx-auto py-8 px-4 flex justify-center">
        <div className="text-muted-foreground">Loading user...</div>
      </div>
    );
  }

  if (!user && !isLoading) {
    return (
      <div className="container mx-auto py-8 px-4 flex justify-center">
        <div className="text-destructive">User not found</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-8 px-4 flex justify-center">
      <UserForm
        user={user}
        onSubmit={handleSubmit}
        onCancel={handleCancel}
        isLoading={isSaving}
        error={error}
      />
    </div>
  );
}
